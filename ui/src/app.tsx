import { component$, useSignal, useStore, $ } from '@builder.io/qwik';
import init, { parse_schedule } from '../pkg/parser_horario.js'; 
import { extractTextFromPDF } from './logic/pdf.js';
import { ScheduleGrid } from './components/ScheduleGrid.tsx';
import { MajorSelector } from './components/MajorSelector.tsx';

import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';

export const App = component$(() => {
    const selectedMajor = useSignal("ICE");
    const scheduleStore = useStore({ encounters: [] });
    const isLoading = useSignal(false);

    const onFileChange = $(async (event: Event) => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        isLoading.value = true;
        try {
            await init();
            const rawText = await extractTextFromPDF(file, pdfWorkerUrl);
            scheduleStore.encounters = parse_schedule(rawText);
        } catch (e) {
            console.error("Error:", e);
        } finally {
            isLoading.value = false;
        }
    });

    return (
        <div class="min-h-screen bg-[#f8fafc] text-slate-900 font-sans selection:bg-blue-100">
            <div class="max-w-[1600px] mx-auto p-4 md:p-8 space-y-6">
                
                {/* Header Section */}
                <header class="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-6 bg-white p-6 rounded-3xl border border-slate-200 shadow-sm">
                    <div class="space-y-1">
                        <h1 class="text-2xl font-black tracking-tight flex items-center gap-2">
                            <span class="bg-blue-600 text-white px-3 py-1 rounded-xl shadow-blue-200 shadow-lg">SIGA</span>
                            <span class="text-slate-800">Parser</span>
                        </h1>
                        <p class="text-slate-400 text-xs font-medium uppercase tracking-widest">Generador de Horarios Offline</p>
                    </div>

                    <div class="flex flex-wrap items-center gap-4 w-full lg:w-auto">
                        <MajorSelector selectedMajor={selectedMajor} />
                        
                        <label class="relative flex-1 lg:flex-none">
                            <input type="file" accept=".pdf" onChange$={onFileChange} class="hidden peer" />
                            <div class="cursor-pointer bg-slate-900 text-slate-50 px-6 py-3 rounded-2xl font-bold text-sm hover:bg-blue-600 hover:shadow-xl hover:shadow-blue-100 transition-all active:scale-95 flex items-center justify-center gap-2">
                                {isLoading.value ? 'Procesando...' : 'Subir Horario .pdf'}
                            </div>
                        </label>
                    </div>
                </header>

                {/* Main Calendar Section */}
                <main class="bg-white rounded-[2.5rem] border border-slate-200 shadow-sm p-2 md:p-6 overflow-hidden">
                    <ScheduleGrid 
                        encounters={scheduleStore.encounters} 
                        filter={selectedMajor.value} 
                    />
                </main>
            </div>
        </div>
    );
});