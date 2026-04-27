import { useState, useMemo, useEffect, useCallback } from 'react';
import init, { parse_schedule } from '../public/pkg/parser_horario.js';
import { extractTextFromPDF } from './logic/pdf.ts';
import { ScheduleGrid } from './components/ScheduleGrid.tsx';
import { MajorSelector } from './components/MajorSelector.tsx';
import { SubjectExplorer } from './components/SubjectExplorer.tsx';
import { getDefaultSelectionIds } from './logic/organizer.ts';
import { getConflictState } from './logic/conflicts.ts';
import { hydrateEncounter, type Encounter } from './logic/domain.ts';

// @ts-ignore
import pdfWorkerUrl from 'pdfjs-dist/build/pdf.worker.mjs?url';

export default function App() {
    const [selectedMajor, setSelectedMajor] = useState("ICE");
    const [isLoading, setIsLoading] = useState(false);
    const [encounters, setEncounters] = useState<Encounter[]>([]);
    const [selectedGroupIds, setSelectedGroupIds] = useState<Set<string>>(new Set());

    // Validación global de conflictos
    const hasConflicts = useMemo(() => {
        const selected = encounters.filter(e => selectedGroupIds.has(e.groupId));
        if (selected.length === 0) return true;

        return selected.some(clase => {
            const state = getConflictState(clase, encounters, selectedGroupIds);
            return state.temporal || state.selection;
        });
    }, [encounters, selectedGroupIds]);

    // Al cambiar la carrera, recalculamos la pre-configuración inteligente
    useEffect(() => {
        if (encounters.length > 0) {
            const carrierEncounters = encounters.filter((e: Encounter) =>
                e.majors_offered.includes(selectedMajor as any)
            );
            setSelectedGroupIds(getDefaultSelectionIds(carrierEncounters));
        }
    }, [selectedMajor, encounters]);

    useEffect(() => {
        document.title = "Mi Itinerario - Wrangler Horarios";
    }, []);

    const onFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (!file) return;

        setIsLoading(true);
        setSelectedGroupIds(new Set()); 

        try {
            await init();
            const rawText = await extractTextFromPDF(file, pdfWorkerUrl);
            const data = parse_schedule(rawText);

            const hydratedEncounters = data.map((e: any) => hydrateEncounter(e));
            setEncounters(hydratedEncounters);

            const carrierEncounters = hydratedEncounters.filter((e: Encounter) =>
                e.majors_offered.includes(selectedMajor as any)
            );
            setSelectedGroupIds(getDefaultSelectionIds(carrierEncounters));

        } catch (e) {
            console.error("Error en el procesamiento:", e);
        } finally {
            setIsLoading(false);
        }
    };

    const toggleSelection = useCallback((groupId: string) => {
        setSelectedGroupIds(prev => {
            const newSet = new Set(prev);
            if (newSet.has(groupId)) {
                newSet.delete(groupId);
            } else {
                const incoming = encounters.find(e => e.groupId === groupId);
                if (incoming) {
                    for (const existingId of newSet) {
                        const existing = encounters.find(e => e.groupId === existingId);
                        if (existing && existing.subject === incoming.subject) {
                            newSet.delete(existingId);
                            break;
                        }
                    }
                }
                newSet.add(groupId);
            }
            return newSet;
        });
    }, [encounters]);

    return (
        <div className="min-h-screen bg-[#f8fafc] text-slate-900 font-sans selection:bg-teal-100">
            <div className="max-w-[100rem] mx-auto p-3 md:p-6 space-y-4">

                <header className="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4 bg-white p-4 rounded-[2rem] border-2 border-slate-100 shadow-sm">
                    <div className="space-y-1">
                        <h1 className="text-xl font-black flex items-center gap-3 tracking-tighter">
                            <span className="bg-teal-700 text-white px-3 py-1 rounded-xl shadow-teal-900/20 shadow-lg">Wrangler</span>
                            <span className="text-slate-900">Horarios</span>
                        </h1>
                    </div>

                    <div className="flex flex-wrap items-center gap-3 w-full lg:w-auto">
                        <MajorSelector selectedMajor={selectedMajor} setSelectedMajor={setSelectedMajor} />

                        <label className="relative flex-1 lg:flex-none group">
                            <input type="file" accept=".pdf" onChange={onFileChange} className="hidden" />
                            <div className="cursor-pointer bg-slate-900 text-slate-50 px-6 py-3 rounded-xl font-black text-[0.6875rem] uppercase tracking-widest hover:bg-teal-700 hover:shadow-2xl hover:shadow-teal-900/20 transition-all active:scale-95 flex items-center justify-center gap-3 overflow-hidden">
                                {isLoading ? (
                                    <span className="flex items-center gap-2">
                                        <div className="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                                        Procesando...
                                    </span>
                                ) : (
                                    <>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline points="17 8 12 3 7 8" /><line x1="12" x2="12" y1="3" y2="15" /></svg>
                                        Subir Horario
                                    </>
                                )}
                            </div>
                        </label>
                    </div>
                </header>

                <div className="grid grid-cols-1 xl:grid-cols-4 gap-4 items-start">
                    <aside className="xl:col-span-1 bg-white rounded-[2rem] border-2 border-slate-100 shadow-sm overflow-hidden flex flex-col h-[calc(100vh-12rem)]">
                        <div className="p-4 border-b border-slate-100 bg-slate-50/50">
                            <div className="flex items-center gap-3 text-slate-800">
                                <h2 className="text-[0.875rem] font-black uppercase tracking-tight flex items-center gap-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round" className="text-teal-600"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z" /><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2Z" /></svg>
                                    Catálogo
                                </h2>
                                <p className="text-[0.625rem] uppercase font-black tracking-widest text-slate-400 pl-3">Asignaturas</p>
                            </div>
                        </div>

                        {encounters.length > 0 ? (
                            <SubjectExplorer
                                encounters={encounters.filter(e => e.majors_offered.includes(selectedMajor as any))}
                                selectedGroupIds={selectedGroupIds}
                                toggleSelection={toggleSelection}
                            />
                        ) : (
                            <div className="flex-1 flex flex-col items-center justify-center text-center p-6 border-2 border-dashed border-slate-100 rounded-2xl m-4">
                                <div className="w-10 h-10 bg-slate-50 rounded-xl flex items-center justify-center mb-3">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-slate-300"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z" /><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2Z" /></svg>
                                </div>
                                <p className="text-[0.75rem] font-bold text-slate-400">Sin datos disponibles</p>
                            </div>
                        )}
                    </aside>

                    <main className="xl:col-span-3 bg-white rounded-[2rem] border-2 border-slate-100 shadow-sm p-3 md:p-6 overflow-hidden">
                        <div className="mb-6 flex justify-between items-end no-print">
                            <div className="space-y-0.5">
                                <h2 className="text-xl font-black text-slate-900 leading-tight">Vista de Itinerario</h2>
                                <p className="text-[0.625rem] uppercase font-black tracking-widest text-teal-600">Plan de Carrera: {selectedMajor}</p>
                            </div>

                            {!hasConflicts && (
                                <button
                                    onClick={() => window.print()}
                                    className="no-print bg-teal-700 text-white px-5 py-2 rounded-xl font-black text-[0.6875rem] uppercase tracking-widest hover:bg-teal-800 transition-all shadow-lg shadow-teal-900/10 flex items-center gap-2 animate-bounce-in"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M6 9V2h12v7" /><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" /><rect width="12" height="8" x="6" y="14" /></svg>
                                </button>
                            )}
                        </div>

                        <ScheduleGrid
                            encounters={encounters}
                            selectedMajor={selectedMajor}
                            selectedGroupIds={selectedGroupIds}
                            toggleSelection={toggleSelection}
                        />
                    </main>
                </div>

            </div>
        </div>
    );
}
