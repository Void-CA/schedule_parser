import { component$, useComputed$, $ } from '@builder.io/qwik';
import { EncounterCard } from './EncounterCard';
import { type Encounter } from '../logic/domain';

interface GridProps {
  encounters: Encounter[];
  selectedMajor: string;
  selectedIds: Set<string>;
  toggleSelection$: (uid: string) => void;
}

const DAYS = ['Hora', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado'];

export const ScheduleGrid = component$(({ 
  encounters, 
  selectedMajor, 
  selectedIds, 
  toggleSelection$ 
}: GridProps) => {

  const displayEncounters = useComputed$(() => {
    // Filtrado por carrera y combinatorio
    const filtered = encounters.filter(e => e.majors_offered.includes(selectedMajor as any));
    return filtered.map(clase => ({
        ...clase,
        isSelected: selectedIds.has(clase.uid!),
        isConflicted: !selectedIds.has(clase.uid!) && false // Lógica de conflicto omitida
    }));
  });

  return (
    <div class="overflow-x-auto custom-scrollbar">
      <div class="grid grid-cols-7 gap-3 min-w-[1100px] bg-white relative p-4 min-h-[600px]">
        
        {/* 1. Cabeceras (Estáticas) */}
        {DAYS.map((day) => (
          <div key={`header-${day}`} class="text-center py-4 text-[10px] font-black text-slate-400 uppercase tracking-[0.2em] border-b border-slate-100">
            {day}
          </div>
        ))}

        {/* 2. Cuerpo del Horario (Dinámico) */}
        {/* Usamos un wrapper lógico para que Qwik no se pierda en el diffing */}
        {displayEncounters.value.length > 0 ? (
          displayEncounters.value.map((clase, idx) => (
            <EncounterCard 
              // Key súper estable y única
              key={`card-${clase.uid}-${idx}`} 
              clase={clase}
              isSelected={clase.isSelected}
              isConflicted={clase.isConflicted}
              toggleSelection$={toggleSelection$}
            />
          ))
        ) : (
          /* Renderizado vacío blindado */
          <div key="empty-state" class="col-span-7 py-32 text-center">
             <p class="text-slate-400">Sube un horario para comenzar</p>
          </div>
        )}

        {/* 3. Guías de fondo */}
        {Array.from({ length: 9 }).map((_, i) => (
          <div key={`guide-${i}`} class="absolute inset-0 pointer-events-none border-b border-slate-900 opacity-[0.03]" style={{ gridRowStart: i + 2, gridColumn: '1 / span 7' }}></div>
        ))}
      </div>
    </div>
  );
});