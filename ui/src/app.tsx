import { component$, useSignal, useStore, $ } from '@builder.io/qwik';
import init, { parse_schedule } from '../pkg/parser_horario.js'; 
import * as pdfjsLib from 'pdfjs-dist';

import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';
import './app.css';

// Ahora configuramos pdf.js para que use ese archivo local empaquetado
pdfjsLib.GlobalWorkerOptions.workerSrc = pdfWorkerUrl;


export const App = component$(() => {
    const selectedMajor = useSignal("ICE");
    // Usamos any temporalmente para las estructuras complejas del WASM
    const scheduleStore = useStore<{ encounters: any[] }>({ encounters: [] });

    const handleFileUpload = $(async (event: Event) => {
        const input = event.target as HTMLInputElement;
        if (!input.files || input.files.length === 0) return;
        
        const file = input.files[0];

        try {
            // 1. Configuramos el worker justo aquí, en el momento exacto en que se necesita
            pdfjsLib.GlobalWorkerOptions.workerSrc = pdfWorkerUrl;

            // 2. Inicializamos WASM
            await init();

            // 3. Leemos el PDF agrupando por líneas físicas
            const arrayBuffer = await file.arrayBuffer();
            const pdf = await pdfjsLib.getDocument({ data: arrayBuffer }).promise;
            let rawText = "";
            
            for (let i = 1; i <= pdf.numPages; i++) {
                const page = await pdf.getPage(i);
                const content = await page.getTextContent();
                
                let lastY = -1;
                let lineText = "";
                
                for (const item of content.items as any[]) {
                    // La posición Y del texto en la página
                    const currentY = item.transform[5];
                    
                    // Si hay un salto en Y mayor a 2 píxeles, cerramos la línea actual
                    if (lastY !== -1 && Math.abs(lastY - currentY) > 2) {
                        rawText += lineText.trim() + "\n";
                        lineText = "";
                    }
                    
                    // Unimos los pedazos de la misma línea con un espacio
                    lineText += item.str + " ";
                    lastY = currentY;
                }
                rawText += lineText.trim() + "\n";
            }

            // Te dejo este console.log para que verifiques que el texto 
            // luce exactamente igual al que veías en la terminal de Rust
            console.log(">>> TEXTO CRUDO:\n", rawText);

            // 4. Invocamos el parser en Rust
            scheduleStore.encounters = parse_schedule(rawText);
            console.log("Horario parseado con éxito:", scheduleStore.encounters);
            
        } catch (error) {
            console.error("Error procesando el horario:", error);
        }
    });

    return (
        <div class="p-6 max-w-6xl mx-auto">
            <header class="mb-8 flex justify-between items-center">
                <h1 class="text-2xl font-bold">Parser de Horarios SIGA</h1>
                <select 
                    bind:value={selectedMajor} 
                    class="border p-2 rounded"
                >
                    <option value="ICE">Ingeniería Cibernética Electrónica</option>
                    <option value="IMS">Ingeniería Mecatrónica</option>
                    <option value="IME">Ingeniería Mecánica</option>
                    <option value="IGI">Ingeniería en Gestión Industrial</option>
                    <option value="IEE">Ingeniería Eléctrica</option>
                    <option value="IEM">Ingeniería Electromédica</option>
                    <option value="LAF">Lic. en Administración Financiera</option>
                    <option value="LCM">Lic. en Comercio y Mercadeo</option>
                </select>
            </header>

            <input 
                type="file" 
                accept="application/pdf" 
                onChange$={handleFileUpload} 
                class="mb-8 block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
            />

            <div class="grid grid-cols-7 gap-4">
                {['Hora', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado'].map(day => (
                    <div key={day} class="font-bold text-center border-b pb-2">{day}</div>
                ))}

                {scheduleStore.encounters
                    .filter((clase: any) => clase.major === selectedMajor.value)
                    .map((clase: any, idx: number) => (
                        <div 
                            key={idx} 
                            class={`col-start-${getDayColumn(clase.day)} row-span-${clase.blocks.length} bg-blue-100 p-2 rounded shadow-sm flex flex-col justify-center`}
                        >
                            <p class="font-bold text-sm leading-tight mb-1">{clase.subject}</p>
                            <p class="text-xs text-gray-700">{clase.room}</p>
                            <p class="text-xs text-gray-500 truncate" title={clase.professor}>{clase.professor}</p>
                        </div>
                ))}
            </div>
        </div>
    );
});

function getDayColumn(dayAbbrev: string): number {
    const dayMap: Record<string, number> = {
        "Monday": 2, "Tuesday": 3, "Wednesday": 4, 
        "Thursday": 5, "Friday": 6, "Saturday": 7
    };
    return dayMap[dayAbbrev] || 2;
}