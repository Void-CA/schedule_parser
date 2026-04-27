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

    const { temporal, selection } = clase.conflictType || { temporal: false, selection: false };

    return (
        <div
            onClick$={() => toggleSelection$(clase.groupId)}
            style={{
                gridColumnStart: getDayColumn(clase.day),
                gridRowStart: getBlockRowStart(clase.blocks),
                gridRowEnd: `span ${clase.blocks.length}`
            }}
            class={[
                "card-base group transition-all duration-300",
                isSelected ? "card-selected" : "opacity-80",
                isConflicted ? "card-conflict" : ""
            ]}
        >
            {/* 1. Encabezado: Materia y Badge de Grupo */}
            <div class="flex justify-between items-start gap-1.5">
                <h4 class="font-black text-[0.6875rem] uppercase leading-tight tracking-tight line-clamp-2 text-inherit">
                    {clase.subject}
                </h4>
                <span class={[
                    "text-[0.5625rem] px-1.5 py-0.5 rounded-md font-black font-mono border",
                    isSelected ? "bg-white/20 border-white/20 text-white" : "bg-slate-50 border-slate-200 text-slate-500"
                ]}>
                    G{clase.group}
                </span>
            </div>

            {/* 2. Cuerpo: Ubicación (Local) */}
            <div class="flex items-center gap-1">
                <div class={[
                    "w-1.5 h-1.5 rounded-full",
                    isSelected ? "bg-teal-300 shadow-[0_0_0.5rem_rgba(94,234,212,0.6)]" : "bg-teal-500"
                ]}></div>
                <p class={[
                    "text-[0.625rem] font-bold tracking-wide",
                    isSelected ? "text-white" : "text-slate-700"
                ]}>
                    Aula {clase.room}
                </p>
            </div>

            {/* 3. Footer: Docente */}
            <div class={[
                "mt-auto pt-1.5 border-t",
                isSelected ? "border-white/10" : "border-slate-100"
            ]}>
                <p class={[
                    "text-[0.5625rem] italic font-medium truncate",
                    isSelected ? "text-white/80" : "text-slate-400"
                ]} title={clase.professor}>
                    {clase.professor}
                </p>
            </div>

            {/* 4. Capas de Alerta de Conflicto */}
            {isConflicted && (
                <div class="absolute top-1.5 right-1.5 flex gap-1 pointer-events-none">
                    {temporal && (
                        <span class="bg-white text-red-600 text-[0.5rem] font-black p-0.5 rounded shadow-md animate-pulse">🕒 BLOQUEADO</span>
                    )}
                    {selection && (
                        <span class="bg-white text-amber-600 text-[0.5rem] font-black p-0.5 rounded shadow-md">📚 DUPLICADO</span>
                    )}
                </div>
            )}
        </div>
    );
});