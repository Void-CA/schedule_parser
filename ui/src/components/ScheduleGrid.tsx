import { component$, useComputed$, $ } from '@builder.io/qwik';
import { EncounterCard } from './EncounterCard';
import { type Encounter } from '../logic/domain';
import { getConflictState } from '../logic/conflicts';

interface GridProps {
  encounters: Encounter[];
  selectedMajor: string;
  selectedGroupIds: Set<string>;
  toggleSelection$: (groupId: string) => void;
}

const FULL_SLOTS = [
    { type: 'academic', range: '08:00 - 08:50 am' },
    { type: 'academic', range: '08:50 - 09:40 am' },
    { type: 'break', label: 'Receso', range: '09:40 - 10:00 am' },
    { type: 'academic', range: '10:00 - 10:50 am' },
    { type: 'academic', range: '10:50 - 11:40 am' },
    { type: 'break', label: 'Almuerzo', range: '11:40 - 01:00 pm' },
    { type: 'academic', range: '01:00 - 01:50 pm' },
    { type: 'academic', range: '01:50 - 02:40 pm' },
    { type: 'break', label: 'Receso', range: '02:40 - 03:00 pm' },
    { type: 'academic', range: '03:00 - 03:50 pm' },
    { type: 'academic', range: '03:50 - 04:40 pm' },
];

const DAYS = ['Hora', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado'];

export const ScheduleGrid = component$(({ 
  encounters, 
  selectedMajor, 
  selectedGroupIds, 
  toggleSelection$ 
}: GridProps) => {

  const displayEncounters = useComputed$(() => {
    const carrierEncounters = encounters.filter(e => e.majors_offered.includes(selectedMajor as any));
    const selected = carrierEncounters.filter(e => selectedGroupIds.has(e.groupId));
    
    return selected.map(clase => {
        const conflicts = getConflictState(clase, encounters, selectedGroupIds);
        return {
            ...clase,
            isSelected: true,
            isConflicted: conflicts.temporal || conflicts.selection,
            conflictType: conflicts
        };
    });
  });

  return (
    <div class="overflow-x-auto custom-scrollbar select-none">
      <div class="grid grid-cols-7 gap-px bg-slate-100 min-w-[1100px] relative border border-slate-200 rounded-3xl overflow-hidden shadow-inner">
        
        {/* 1. Cabeceras */}
        {DAYS.map((day, i) => (
          <div key={`header-${day}`} class={[
            "text-center py-3 text-[10px] font-black text-slate-500 uppercase tracking-[0.2em] bg-white border-b border-slate-100",
            i === 0 ? "bg-slate-50/80" : ""
          ]}>
            {day}
          </div>
        ))}

        {/* 2. Marcadores de Tiempo (Estructura de 11 filas) */}
        {FULL_SLOTS.map((slot, i) => (
            <div key={`slot-${i}`} 
                class={[
                    "time-slot-label px-2 space-y-0.5",
                    slot.type === 'break' ? "py-2 bg-slate-200" : "py-3 bg-white"
                ]}
                style={{ gridRowStart: i + 2, gridColumnStart: 1 }}
            >
                <div class="flex flex-col items-center leading-none text-center">
                    <span class={[
                        "text-[9px] font-black",
                        slot.type === 'break' ? "text-slate-500" : "text-teal-800"
                    ]}>
                        {slot.range.split(' - ')[0].replace(' am', '').replace(' pm', '')}
                    </span>
                    <span class="text-[7px] opacity-30">—</span>
                    <span class={[
                        "text-[9px] font-black",
                        slot.type === 'break' ? "text-slate-500" : "text-teal-800"
                    ]}>
                        {slot.range.split(' - ')[1].replace(' am', '').replace(' pm', '')}
                    </span>
                </div>
            </div>
        ))}

        {/* 3. Guías de Fondo y Bloqueos Visuales de Recesos */}
        {Array.from({ length: 11 }).map((_, row) => {
            const isBreak = FULL_SLOTS[row].type === 'break';
            return (
                <>
                    {/* Celdas de Guía */}
                    {Array.from({ length: 6 }).map((_, col) => (
                        <div key={`cell-${row}-${col}`} 
                            class={[
                                "border-[0.5px] border-slate-50",
                                isBreak ? "bg-slate-100 bg-repeating-dots opacity-40" : "bg-white/50"
                            ]}
                            style={{ gridRowStart: row + 2, gridColumnStart: col + 2 }}
                        ></div>
                    ))}
                    
                    {/* Overlay de Texto para Recesos */}
                    {isBreak && (
                        <div 
                            class="pointer-events-none flex items-center justify-center z-10"
                            style={{ gridRowStart: row + 2, gridColumnStart: 2, gridColumnEnd: 'span 6' }}
                        >
                            <span class="text-[16px] font-black text-slate-500 uppercase tracking-[2em] whitespace-nowrap pl-[2em]">
                                {FULL_SLOTS[row].label}
                            </span>
                        </div>
                    )}
                </>
            );
        })}

        {/* 4. Encuentros Seleccionados */}
        {displayEncounters.value.length > 0 && 
          displayEncounters.value.map((clase, idx) => (
            <EncounterCard 
                key={`card-${clase.uid}-${idx}`} 
                clase={clase}
                isSelected={clase.isSelected}
                isConflicted={clase.isConflicted}
                toggleSelection$={toggleSelection$}
            />
          ))
        }

        {displayEncounters.value.length === 0 && (
          <div key="empty-state" class="col-span-6 col-start-2 row-span-11 flex flex-col items-center justify-center bg-white/50 backdrop-blur-sm z-30">
             <div class="bg-slate-100 p-6 rounded-full mb-4 animate-pulse">
                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-slate-300"><rect width="18" height="18" x="3" y="4" rx="2" ry="2"/><line x1="16" x2="16" y1="2" y2="6"/><line x1="8" x2="8" y1="2" y2="6"/><line x1="3" x2="21" y1="10" y2="10"/><path d="m9 16 2 2 4-4"/></svg>
             </div>
             <p class="text-slate-400 font-black uppercase text-[10px] tracking-[0.2em]">Selecciona materias en el catálogo lateral</p>
          </div>
        )}
      </div>
    </div>
  );
});