import { getBlockRowStart, getDayColumn } from '../logic/utils.ts';
import { type Encounter } from '../logic/domain.ts';

interface EncounterCardProps {
    clase: Encounter;
    isSelected: boolean;
    isConflicted: boolean;
    toggleSelection: (groupId: string) => void;
}

export const EncounterCard = ({
    clase,
    isSelected,
    isConflicted,
    toggleSelection
}: EncounterCardProps) => {

    const { temporal, selection } = clase.conflictType || { temporal: false, selection: false };

    return (
        <div
            onClick={() => toggleSelection(clase.groupId)}
            style={{
                gridColumnStart: getDayColumn(clase.day),
                gridRowStart: getBlockRowStart(clase.blocks),
                gridRowEnd: `span ${clase.blocks.length}`
            }}
            className={`card-base group transition-all duration-300 ${
                isSelected ? "card-selected" : "opacity-80"
            } ${
                isConflicted ? "card-conflict" : ""
            }`}
        >
            {/* 1. Encabezado: Materia y Badge de Grupo */}
            <div className="flex justify-between items-start gap-1.5">
                <h4 className="font-black text-[0.6875rem] uppercase leading-tight tracking-tight line-clamp-2 text-inherit">
                    {clase.subject}
                </h4>
                <span className={`text-[0.5625rem] px-1.5 py-0.5 rounded-md font-black font-mono border ${
                    isSelected ? "bg-white/20 border-white/20 text-white" : "bg-slate-50 border-slate-200 text-slate-500"
                }`}>
                    G{clase.group}
                </span>
            </div>

            {/* 2. Cuerpo: Ubicación (Local) */}
            <div className="flex items-center gap-1">
                <div className={`w-1.5 h-1.5 rounded-full ${
                    isSelected ? "bg-teal-300 shadow-[0_0_0.5rem_rgba(94,234,212,0.6)]" : "bg-teal-500"
                }`}></div>
                <p className={`text-[0.625rem] font-bold tracking-wide ${
                    isSelected ? "text-white" : "text-slate-700"
                }`}>
                    Aula {clase.room}
                </p>
            </div>

            {/* 3. Footer: Docente */}
            <div className={`mt-auto pt-1.5 border-t ${
                isSelected ? "border-white/10" : "border-slate-100"
            }`}>
                <p className={`text-[0.5625rem] italic font-medium truncate ${
                    isSelected ? "text-white/80" : "text-slate-400"
                }`} title={clase.professor}>
                    {clase.professor}
                </p>
            </div>

            {/* 4. Capas de Alerta de Conflicto */}
            {isConflicted && (
                <div className="absolute top-1.5 right-1.5 flex gap-1 pointer-events-none">
                    {temporal && (
                        <span className="bg-white text-red-600 text-[0.5rem] font-black p-0.5 rounded shadow-md animate-pulse">🕒 BLOQUEADO</span>
                    )}
                    {selection && (
                        <span className="bg-white text-amber-600 text-[0.5rem] font-black p-0.5 rounded shadow-md">📚 DUPLICADO</span>
                    )}
                </div>
            )}
        </div>
    );
};