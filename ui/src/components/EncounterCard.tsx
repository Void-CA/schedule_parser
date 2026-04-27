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

    return (
        <div 
            onClick$={() => toggleSelection$(clase.uid!)}
            style={{ 
                gridColumnStart: getDayColumn(clase.day), 
                gridRowStart: getBlockRowStart(clase.blocks),
                gridRowEnd: `span ${clase.blocks.length}`
            }}
            class={[
                "card-base group",
                isSelected ? "card-selected" : "",
                isConflicted && !isSelected ? "card-conflict" : "",
            ]}
        >
            {/* Header: Materia y Grupo */}
            <div class="space-y-1">
                <div class="flex justify-between items-start gap-1">
                    <h4 class="font-bold text-xs uppercase leading-tight line-clamp-2">
                        {clase.subject}
                    </h4>
                    {!isSelected && isConflicted && (
                        <span class="text-slate-400 text-[10px]" title="Choque de horario">⚠️</span>
                    )}
                </div>
                
                <div class="flex flex-wrap gap-1 mt-1">
                    <span class={[
                        "text-[9px] px-1.5 py-0.5 rounded font-mono font-medium",
                        isSelected ? "bg-teal-700 text-teal-50" : "bg-slate-100 text-slate-600"
                    ]}>
                        G{clase.group}
                    </span>
                    <span class={[
                        "text-[9px] px-1.5 py-0.5 rounded font-medium",
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
                    "text-[10px] truncate",
                    isSelected ? "text-teal-800" : "text-slate-500 group-hover:text-slate-700"
                ]} title={clase.professor}>
                    {clase.professor}
                </p>
            </div>

            {/* Indicador visual de Seleccionado */}
            {isSelected && (
                <div class="absolute -right-2 -bottom-2 opacity-10 text-teal-900">
                    <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                        <polyline points="20 6 9 17 4 12"></polyline>
                    </svg>
                </div>
            )}
        </div>
    );
});