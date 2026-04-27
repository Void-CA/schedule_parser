import { component$, useComputed$ } from '@builder.io/qwik';
import { type Encounter } from '../logic/domain';
import { groupEncountersBySubject } from '../logic/organizer';

interface ExplorerProps {
    encounters: Encounter[];
    selectedGroupIds: Set<string>;
    toggleSelection$: (groupId: string) => void;
}

export const SubjectExplorer = component$(({ 
    encounters, 
    selectedGroupIds, 
    toggleSelection$ 
}: ExplorerProps) => {

    const grouped = useComputed$(() => groupEncountersBySubject(encounters));

    return (
        <div class="space-y-4 max-h-[calc(100vh-250px)] overflow-y-auto pr-2 custom-scrollbar">
            {grouped.value.map((subject) => (
                <div key={subject.subject} class="bg-white border-2 border-slate-100 rounded-2xl p-4 space-y-3 transition-all hover:border-teal-100 hover:shadow-sm">
                    <div class="flex justify-between items-center border-b border-slate-50 pb-2">
                        <h3 class="text-[10px] font-black uppercase tracking-wider text-slate-800 truncate max-w-[200px]">
                            {subject.subject}
                        </h3>
                        <span class="text-[9px] font-bold text-slate-400 bg-slate-50 px-1.5 py-0.5 rounded uppercase">
                            {subject.options.length} opciones
                        </span>
                    </div>
                    
                    <div class="flex flex-wrap gap-2">
                        {subject.options.map((option) => {
                            const isSelected = selectedGroupIds.has(option.groupId);
                            return (
                                <button
                                    key={option.groupId}
                                    onClick$={() => toggleSelection$(option.groupId)}
                                    class={[
                                        "px-3 py-1.5 rounded-xl text-[10px] font-black transition-all active:scale-95",
                                        isSelected 
                                            ? "bg-teal-700 text-white shadow-lg shadow-teal-100 scale-105" 
                                            : "bg-slate-50 text-slate-500 hover:bg-slate-100 border border-slate-200"
                                    ]}
                                >
                                    G{option.groupNumber}
                                </button>
                            );
                        })}
                    </div>
                </div>
            ))}
        </div>
    );
});
