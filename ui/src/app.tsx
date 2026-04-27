import { component$, useSignal, useStore, $ } from '@builder.io/qwik';
import init, { parse_schedule } from '../pkg/parser_horario.js'; 
import * as pdfjsLib from 'pdfjs-dist';
import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';
import { extractTextFromPDF } from './logic/pdf.js';
import { getBlockRowStart, getDayColumn } from './logic/utils.js';

// Configuración Global
pdfjsLib.GlobalWorkerOptions.workerSrc = pdfWorkerUrl;

const MAJORS = [
    { id: "ICE", name: "Ing. Cibernética Electrónica" },
    { id: "IMS", name: "Ing. Mecatrónica" },
    { id: "IME", name: "Ing. Mecánica" },
    { id: "IGI", name: "Ing. Gestión Industrial" },
    { id: "IEE", name: "Ing. Eléctrica" },
    { id: "IEM", name: "Ing. Electromédica" },
    { id: "LAF", name: "Lic. Administración Financiera" },
    { id: "LCM", name: "Lic. Comercio y Mercadeo" },
];

const DAYS = ['Hora', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado'];

export const App = component$(() => {
    const selectedMajor = useSignal("ICE");
    const scheduleStore = useStore<{ encounters: any[] }>({ encounters: [] });

    const onFileChange = $(async (event: Event) => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        try {
            await init();
            const rawText = await extractTextFromPDF(file);
            scheduleStore.encounters = parse_schedule(rawText);
        } catch (e) {
            console.error("Error en el procesamiento:", e);
        }
    });

    return (
        <div class="p-6 max-w-7xl mx-auto space-y-8">
            <header class="flex flex-col md:flex-row justify-between items-center gap-4 bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
                <h1 class="text-3xl font-black text-gray-800 tracking-tight">SIGA <span class="text-blue-600">Parser</span></h1>
                
                <div class="flex flex-wrap gap-3">
                    <select bind:value={selectedMajor} class="select-style">
                        {MAJORS.map(m => <option key={m.id} value={m.id}>{m.name}</option>)}
                    </select>
                    <input type="file" accept=".pdf" onChange$={onFileChange} class="file-input-style" />
                </div>
            </header>

            <main class="grid grid-cols-7 gap-3 bg-gray-50 p-6 rounded-3xl border border-gray-200 shadow-inner overflow-x-auto min-w-[1000px]">
                {DAYS.map(day => (
                    <div key={day} class="text-center font-bold text-gray-400 uppercase text-xs tracking-widest mb-4">
                        {day}
                    </div>
                ))}

                {scheduleStore.encounters
                    .filter(e => e.major === selectedMajor.value)
                    .map((clase, idx) => (
                        <div 
                            key={idx} 
                            style={{ 
                                gridColumnStart: getDayColumn(clase.day), 
                                gridRowStart: getBlockRowStart(clase.blocks),
                                gridRowEnd: `span ${clase.blocks.length}`
                            }}
                            class="encounter-card group"
                        >
                            <div class="flex justify-between items-start">
                                <span class="font-black text-[11px] uppercase leading-none">{clase.subject}</span>
                                <span class="bg-blue-600 text-white text-[8px] px-1.5 py-0.5 rounded-full font-bold">G{clase.group}</span>
                            </div>
                            <span class="text-[10px] font-mono text-blue-700 font-bold mt-2">{clase.room}</span>
                            <span class="text-[9px] text-gray-500 truncate mt-1 italic opacity-0 group-hover:opacity-100 transition-opacity">
                                {clase.professor}
                            </span>
                        </div>
                ))}
            </main>
        </div>
    );
});