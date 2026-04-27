import { component$, useSignal, useStore, $, useTask$, useComputed$, useVisibleTask$ } from '@builder.io/qwik';
import init, { parse_schedule } from '../pkg/parser_horario.js';
import { extractTextFromPDF } from './logic/pdf.js';
import { ScheduleGrid } from './components/ScheduleGrid.tsx';
import { MajorSelector } from './components/MajorSelector.tsx';
import { SubjectExplorer } from './components/SubjectExplorer.tsx';
import { getDefaultSelectionIds } from './logic/organizer';
import { getConflictState } from './logic/conflicts';
import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';
import { hydrateEncounter, type Encounter } from './logic/domain';

export const App = component$(() => {
    const selectedMajor = useSignal("ICE");
    const isLoading = useSignal(false);

    // Store principal para los datos del WASM
    const scheduleStore = useStore<{ encounters: Encounter[] }>({
        encounters: []
    });

    // Store para el combinatorio (IDs de Grupos seleccionados por el usuario)
    const selectionStore = useStore<{ selectedGroupIds: Set<string> }>({
        selectedGroupIds: new Set()
    });

    const hasConflicts = useComputed$(() => {
        const selected = scheduleStore.encounters.filter(e =>
            selectionStore.selectedGroupIds.has(e.groupId)
        );
        if (selected.length === 0) return true;

        return selected.some(clase => {
            const state = getConflictState(clase, scheduleStore.encounters, selectionStore.selectedGroupIds);
            return state.temporal || state.selection;
        });
    });

    /**
     * Reactividad: Al cambiar la carrera, recalculamos la pre-configuración inteligente (G1 por defecto)
     */
    useTask$(({ track }) => {
        track(() => selectedMajor.value);
        if (scheduleStore.encounters.length > 0) {
            const carrierEncounters = scheduleStore.encounters.filter((e: Encounter) =>
                e.majors_offered.includes(selectedMajor.value as any)
            );
            selectionStore.selectedGroupIds = getDefaultSelectionIds(carrierEncounters);
        }
    });


    useVisibleTask$(() => {
        document.title = "Mi Itinerario - WHorarios";
    });

    /**
     * Acción al subir el archivo: Limpia selecciones previas y parsea el PDF.
     */
    const onFileChange = $(async (event: Event) => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        isLoading.value = true;
        selectionStore.selectedGroupIds.clear(); // Limpiamos selección al cargar nuevo PDF

        try {
            await init();
            const rawText = await extractTextFromPDF(file, pdfWorkerUrl);
            const data = parse_schedule(rawText);

            // Hidratamos los encuentros (calcula GID y UID internamente)
            const hydratedEncounters = data.map((e: any) => hydrateEncounter(e));
            scheduleStore.encounters = hydratedEncounters;

            // --- Lógica de Pre-Configuración Inteligente ---
            // Solo para la carrera seleccionada
            const carrierEncounters = hydratedEncounters.filter((e: Encounter) =>
                e.majors_offered.includes(selectedMajor.value as any)
            );
            selectionStore.selectedGroupIds = getDefaultSelectionIds(carrierEncounters);

        } catch (e) {
            console.error("Error en el procesamiento:", e);
        } finally {
            isLoading.value = false;
        }
    });

    /**
     * Lógica del Combinatorio: Alternar selección de clases.
     */
    /**
     * Lógica del Combinatorio: Alternar selección por GRUPO.
     */
    const toggleSelection = $((groupId: string) => {
        const newSet = new Set(selectionStore.selectedGroupIds);

        if (newSet.has(groupId)) {
            newSet.delete(groupId);
        } else {
            // Regla de Exclusividad: Buscar si ya hay un grupo de esta materia seleccionado
            const incoming = scheduleStore.encounters.find(e => e.groupId === groupId);
            if (incoming) {
                for (const existingId of newSet) {
                    const existing = scheduleStore.encounters.find(e => e.groupId === existingId);
                    if (existing && existing.subject === incoming.subject) {
                        newSet.delete(existingId);
                        break; // Solo puede haber uno por materia
                    }
                }
            }
            newSet.add(groupId);
        }
        selectionStore.selectedGroupIds = newSet;
    });

    return (
        <div class="min-h-screen bg-[#f8fafc] text-slate-900 font-sans selection:bg-teal-100">
            <div class="max-w-[100rem] mx-auto p-3 md:p-6 space-y-4">

                {/* Header: Identidad y Controles */}
                <header class="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4 bg-white p-4 rounded-[2rem] border-2 border-slate-100 shadow-sm">
                    <div class="space-y-1">
                        <h1 class="text-xl font-black flex items-center gap-3 tracking-tighter">
                            <span class="bg-teal-700 text-white px-3 py-1 rounded-xl shadow-teal-900/20 shadow-lg">Wrangler</span>
                            <span class="text-slate-900">Horarios</span>
                        </h1>
                    </div>

                    <div class="flex flex-wrap items-center gap-3 w-full lg:w-auto">
                        <MajorSelector selectedMajor={selectedMajor} />

                        <label class="relative flex-1 lg:flex-none group">
                            <input type="file" accept=".pdf" onChange$={onFileChange} class="hidden" />
                            <div class="cursor-pointer bg-slate-900 text-slate-50 px-6 py-3 rounded-xl font-black text-[0.6875rem] uppercase tracking-widest hover:bg-teal-700 hover:shadow-2xl hover:shadow-teal-900/20 transition-all active:scale-95 flex items-center justify-center gap-3 overflow-hidden">
                                {isLoading.value ? (
                                    <span class="flex items-center gap-2">
                                        <div class="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                                        Procesando...
                                    </span>
                                ) : (
                                    <>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline points="17 8 12 3 7 8" /><line x1="12" x2="12" y1="3" y2="15" /></svg>
                                        Subir Horario
                                    </>
                                )}
                            </div>
                        </label>
                    </div>
                </header>

                <div class="grid grid-cols-1 xl:grid-cols-4 gap-4 items-start">
                    {/* Panel Izquierdo: Explorador de Materias */}
                    <aside class="xl:col-span-1 bg-white rounded-[2rem] border-2 border-slate-100 shadow-sm overflow-hidden flex flex-col h-[calc(100vh-12rem)]">
                        <div class="p-4 border-b border-slate-100 bg-slate-50/50">
                            <div class="flex items-center gap-3 text-slate-800">
                                <h2 class="text-[0.875rem] font-black uppercase tracking-tight flex items-center gap-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="text-teal-600"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z" /><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2Z" /></svg>
                                    Catálogo
                                </h2>
                                <p class="text-[0.625rem] uppercase font-black tracking-widest text-slate-400 pl-3">Asignaturas</p>
                            </div>
                        </div>

                        {scheduleStore.encounters.length > 0 ? (
                            <SubjectExplorer
                                encounters={scheduleStore.encounters.filter(e => e.majors_offered.includes(selectedMajor.value as any))}
                                selectedGroupIds={selectionStore.selectedGroupIds}
                                toggleSelection$={toggleSelection}
                            />
                        ) : (
                            <div class="flex-1 flex flex-col items-center justify-center text-center p-6 border-2 border-dashed border-slate-100 rounded-2xl m-4">
                                <div class="w-10 h-10 bg-slate-50 rounded-xl flex items-center justify-center mb-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-slate-300"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z" /><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2Z" /></svg>
                                </div>
                                <p class="text-[0.75rem] font-bold text-slate-400">Sin datos disponibles</p>
                            </div>
                        )}
                    </aside>

                    {/* Panel Derecho: Visualizador de Horario */}
                    <main class="xl:col-span-3 bg-white rounded-[2rem] border-2 border-slate-100 shadow-sm p-3 md:p-6 overflow-hidden">
                        <div class="mb-6 flex justify-between items-end no-print">
                            <div class="space-y-0.5">
                                <h2 class="text-xl font-black text-slate-900 leading-tight">Vista de Itinerario</h2>
                                <p class="text-[0.625rem] uppercase font-black tracking-widest text-teal-600">Plan de Carrera: {selectedMajor.value}</p>
                            </div>

                            {!hasConflicts.value && (
                                <button
                                    onClick$={() => window.print()}
                                    class="no-print bg-teal-700 text-white px-5 py-2 rounded-xl font-black text-[0.6875rem] uppercase tracking-widest hover:bg-teal-800 transition-all shadow-lg shadow-teal-900/10 flex items-center gap-2 animate-bounce-in"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><path d="M6 9V2h12v7" /><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" /><rect width="12" height="8" x="6" y="14" /></svg>
                                </button>
                            )}
                        </div>

                        <ScheduleGrid
                            encounters={scheduleStore.encounters}
                            selectedMajor={selectedMajor.value}
                            selectedGroupIds={selectionStore.selectedGroupIds}
                            toggleSelection$={toggleSelection}
                        />
                    </main>
                </div>

            </div>
        </div>
    );
});