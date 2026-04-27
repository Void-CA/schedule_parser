import { component$, useSignal, useStore, $, useComputed$ } from '@builder.io/qwik';
import init, { parse_schedule } from '../pkg/parser_horario.js'; 
import { extractTextFromPDF } from './logic/pdf.js';
import { ScheduleGrid } from './components/ScheduleGrid.tsx';
import { MajorSelector } from './components/MajorSelector.tsx';
import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';
import { hydrateEncounter, type Encounter } from './logic/domain';

export const App = component$(() => {
    const selectedMajor = useSignal("ICE");
    const isLoading = useSignal(false);
    
    // Store principal para los datos del WASM
    const scheduleStore = useStore<{ encounters: Encounter[] }>({ 
        encounters: [] 
    });

    // Store para el combinatorio (IDs de clases seleccionadas por el usuario)
    const selectionStore = useStore<{ selectedIds: Set<string> }>({ 
        selectedIds: new Set() 
    });

    /** * Función para generar un ID único por encuentro. 
     * Útil ya que una materia puede tener varios grupos o días.
     */
    const getEncounterId = (e: Encounter) => 
        `${e.subject}-${e.group}-${e.day}-${e.blocks.join('')}`;

    /**
     * Acción al subir el archivo: Limpia selecciones previas y parsea el PDF.
     */
    const onFileChange = $(async (event: Event) => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        isLoading.value = true;
        selectionStore.selectedIds.clear(); // Limpiamos selección al cargar nuevo PDF

        try {
            await init();
            const rawText = await extractTextFromPDF(file, pdfWorkerUrl);
            const data = parse_schedule(rawText);
            
            // Inyectamos un ID único a cada objeto antes de guardarlo e hidratamos
            scheduleStore.encounters = data.map((e: any) => {
                const hydrated = hydrateEncounter(e);
                hydrated.uid = getEncounterId(hydrated);
                return hydrated;
            });
        } catch (e) {
            console.error("Error en el procesamiento:", e);
        } finally {
            isLoading.value = false;
        }
    });

    /**
     * Lógica del Combinatorio: Alternar selección de clases.
     */
    const toggleSelection = $((uid: string) => {
        const newSet = new Set(selectionStore.selectedIds);
        if (newSet.has(uid)) {
            newSet.delete(uid);
        } else {
            newSet.add(uid);
        }
        selectionStore.selectedIds = newSet;
    });

    return (
        <div class="min-h-screen bg-[#f8fafc] text-slate-900 font-sans selection:bg-teal-100">
            <div class="max-w-[1600px] mx-auto p-4 md:p-8 space-y-6">
                
                {/* Header: Identidad y Controles */}
                <header class="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-6 bg-white p-6 rounded-3xl border border-slate-200 shadow-sm">
                    <div class="space-y-1">
                        <h1 class="text-2xl font-black tracking-tight flex items-center gap-2">
                            <span class="bg-teal-600 text-white px-3 py-1 rounded-xl shadow-teal-200 shadow-lg">SIGA</span>
                            <span class="text-slate-800">Parser</span>
                        </h1>
                        <div class="flex items-center gap-2">
                            <span class="relative flex h-2 w-2">
                                <span class={`animate-ping absolute inline-flex h-full w-full rounded-full ${scheduleStore.encounters.length > 0 ? 'bg-green-400' : 'bg-slate-400'} opacity-75`}></span>
                                <span class={`relative inline-flex rounded-full h-2 w-2 ${scheduleStore.encounters.length > 0 ? 'bg-green-500' : 'bg-slate-500'}`}></span>
                            </span>
                            <p class="text-slate-400 text-xs font-medium uppercase tracking-widest">
                                {scheduleStore.encounters.length > 0 
                                    ? `${scheduleStore.encounters.length} encuentros cargados` 
                                    : 'Esperando PDF...'}
                            </p>
                        </div>
                    </div>

                    <div class="flex flex-wrap items-center gap-4 w-full lg:w-auto">
                        <MajorSelector selectedMajor={selectedMajor} />
                        
                        <label class="relative flex-1 lg:flex-none group">
                            <input type="file" accept=".pdf" onChange$={onFileChange} class="hidden" />
                            <div class="cursor-pointer bg-slate-900 text-slate-50 px-6 py-3 rounded-2xl font-bold text-sm hover:bg-teal-600 hover:shadow-xl hover:shadow-teal-100 transition-all active:scale-95 flex items-center justify-center gap-2 overflow-hidden relative">
                                {isLoading.value ? (
                                    <span class="flex items-center gap-2">
                                        <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                                        Procesando...
                                    </span>
                                ) : (
                                    <>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/></svg>
                                        Subir Horario .pdf
                                    </>
                                )}
                            </div>
                        </label>
                    </div>
                </header>

                {/* Main Calendar Section */}
                <main class="bg-white rounded-[2.5rem] border border-slate-200 shadow-sm p-2 md:p-6 overflow-hidden">
                    <ScheduleGrid 
                        encounters={scheduleStore.encounters} 
                        selectedMajor={selectedMajor.value}
                        selectedIds={selectionStore.selectedIds}
                        toggleSelection$={toggleSelection}
                    />
                </main>
                
                {/* Status Bar: Resumen del Combinatorio */}
                {selectionStore.selectedIds.size > 0 && (
                    <footer class="fixed bottom-6 left-1/2 -translate-x-1/2 bg-slate-900 text-white px-6 py-3 rounded-full shadow-2xl flex items-center gap-4 animate-bounce-in z-50 border border-white/10">
                        <p class="text-sm font-bold">
                            {selectionStore.selectedIds.size} clases seleccionadas
                        </p>
                        <div class="h-4 w-[1px] bg-white/20"></div>
                        <button 
                            onClick$={() => selectionStore.selectedIds.clear()}
                            class="text-xs uppercase tracking-widest font-black text-red-400 hover:text-red-300 transition-colors"
                        >
                            Limpiar
                        </button>
                    </footer>
                )}
            </div>
        </div>
    );
});