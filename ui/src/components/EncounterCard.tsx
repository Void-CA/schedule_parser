import { component$ } from '@builder.io/qwik';
import { getBlockRowStart, getDayColumn } from '../logic/utils.js';
import { type Encounter } from '../logic/domain';

interface EncounterCardProps {
    clase: Encounter;
    isSelected: boolean;
    isConflicted: boolean;
    toggleSelection$: (uid: string) => void;
}

export const EncounterCard = component$(({ 
    clase, 
    isSelected, 
    isConflicted, 
    toggleSelection$ 
}: EncounterCardProps) => {

    const conflictDetails = (clase as any).conflictType;

    return (
        <div 
            onClick$={() => toggleSelection$(clase.groupId)}
            style={{ 
                gridColumnStart: getDayColumn(clase.day), 
                gridRowStart: getBlockRowStart(clase.blocks),
                gridRowEnd: `span ${clase.blocks.length}`
            }}
            class={[
                "card-base group relative transition-all duration-300 ease-out cursor-pointer",
                isSelected ? "card-selected scale-[1.02] shadow-xl z-10" : "hover:scale-[1.01] hover:shadow-md",
                isConflicted ? (isSelected ? "border-red-500 ring-2 ring-red-500/20" : "card-conflict opacity-80") : "",
            ]}
        >
            {/* Header: Materia y Grupo */}
            <div class="space-y-1">
                <div class="flex justify-between items-start gap-1">
                    <h4 class={[
                        "font-bold text-[11px] uppercase leading-tight line-clamp-2 transition-colors",
                        isSelected ? "text-teal-900" : "text-slate-800"
                    ]}>
                        {clase.subject}
                    </h4>
                    {isConflicted && (
                        <div class="flex gap-0.5">
                           {conflictDetails?.temporal && <span class="text-red-500 text-[10px] animate-pulse" title="Traslape de horario">🕒</span>}
                           {conflictDetails?.selection && <span class="text-amber-500 text-[10px]" title="Materia ya seleccionada en otro grupo">📚</span>}
                        </div>
                    )}
                </div>
                
                <div class="flex flex-wrap gap-1 mt-1">
                    <span class={[
                        "text-[9px] px-1.5 py-0.5 rounded-md font-mono font-bold transition-colors",
                        isSelected ? "bg-teal-700 text-teal-50" : "bg-slate-100 text-slate-600"
                    ]}>
                        G{clase.group}
                    </span>
                    <span class={[
                        "text-[9px] px-1.5 py-0.5 rounded-md font-medium transition-colors",
                        isSelected ? "bg-teal-200 text-teal-900" : "bg-slate-100 text-slate-600"
                    ]}>
                        {clase.room}
                    </span>
                </div>
            </div>

            {/* Footer: Información del Docente */}
            <div class={[
                "mt-auto pt-1 border-t transition-colors duration-200",
                isSelected ? "border-teal-500/30" : "border-slate-100",
            ]}>
                <p class={[
                    "text-[10px] truncate font-medium",
                    isSelected ? "text-teal-800" : "text-slate-400 group-hover:text-slate-600"
                ]} title={clase.professor}>
                    {clase.professor}
                </p>
            </div>

            {/* Indicador visual de Seleccionado */}
            {isSelected && (
                <div class="absolute -right-1 -bottom-1 opacity-20 text-teal-900">
                    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                        <polyline points="20 6 9 17 4 12"></polyline>
                    </svg>
                </div>
            )}
        </div>
    );
});