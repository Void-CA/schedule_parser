import { useMemo } from 'react';
import { type Encounter } from '../logic/domain.ts';
import { groupEncountersBySubject } from '../logic/organizer.ts';

interface ExplorerProps {
    encounters: Encounter[];
    selectedGroupIds: Set<string>;
    toggleSelection: (groupId: string) => void;
}

export const SubjectExplorer = ({ 
    encounters, 
    selectedGroupIds, 
    toggleSelection 
}: ExplorerProps) => {

    const grouped = useMemo(() => groupEncountersBySubject(encounters), [encounters]);

    return (
        <div className="space-y-3 max-h-full overflow-y-auto pr-2 custom-scrollbar">
            {grouped.map((subject) => (
                <div key={subject.subject} className="bg-white border-2 border-slate-100 rounded-xl p-3 space-y-2.5 transition-all hover:border-teal-100 hover:shadow-sm">
                    <div className="flex justify-between items-center border-b border-slate-50 pb-1.5">
                        <h3 className="text-[0.625rem] font-black uppercase tracking-wider text-slate-800 truncate max-w-[12rem]">
                            {subject.subject}
                        </h3>
                        <span className="text-[0.5625rem] font-bold text-slate-400 bg-slate-50 px-1.5 py-0.5 rounded uppercase">
                            {subject.options.length} opciones
                        </span>
                    </div>
                    
                    <div className="flex flex-wrap gap-2">
                        {subject.options.map((option) => {
                            const isSelected = selectedGroupIds.has(option.groupId);
                            return (
                                <button
                                    key={option.groupId}
                                    onClick={() => toggleSelection(option.groupId)}
                                    className={`px-2.5 py-1.5 rounded-lg text-[0.625rem] font-black transition-all active:scale-95 ${
                                        isSelected 
                                            ? "bg-teal-700 text-white shadow-lg shadow-teal-100 scale-105" 
                                            : "bg-slate-50 text-slate-500 hover:bg-slate-100 border border-slate-200"
                                    }`}
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
};
