import * as pdfjsLib from 'pdfjs-dist';

export const extractTextFromPDF = async (file: File, workerUrl: string): Promise<string> => {
    // Configuración interna obligatoria
    pdfjsLib.GlobalWorkerOptions.workerSrc = workerUrl;

    const arrayBuffer = await file.arrayBuffer();
    const pdf = await pdfjsLib.getDocument({ data: arrayBuffer }).promise;
    let fullText = "";

    for (let i = 1; i <= pdf.numPages; i++) {
        const page = await pdf.getPage(i);
        const content = await page.getTextContent();
        
        let lastY = -1;
        let lineText = "";
        
        for (const item of content.items as any[]) {
            const currentY = item.transform[5];
            if (lastY !== -1 && Math.abs(lastY - currentY) > 2) {
                fullText += lineText.trim() + "\n";
                lineText = "";
            }
            lineText += item.str + " ";
            lastY = currentY;
        }
        fullText += lineText.trim() + "\n";
    }
    return fullText;
};