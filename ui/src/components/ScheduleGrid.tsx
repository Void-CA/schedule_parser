import { component$ } from '@builder.io/qwik';
import { EncounterCard } from './EncounterCard';

interface GridProps {
  encounters: any[];
  filter: string;
}

const DAYS = ['Hora', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado'];

export const ScheduleGrid = component$(({ encounters, filter }: GridProps) => {
  // Filtramos los encuentros antes de renderizar para mantener el JSX limpio
  const filteredEncounters = encounters.filter(e => e.major === filter);

  return (
    <div class="overflow-x-auto custom-scrollbar">
      {/* Definimos el Grid base con 7 columnas y un alto mínimo para que los bloques respiren */}
      <div class="grid grid-cols-7 gap-3 min-w-[1000px] bg-white relative">
        
        {/* Renderizado de las etiquetas de los días (Fila 1) */}
        {DAYS.map((day) => (
          <div 
            key={day} 
            class="text-center py-4 text-[10px] font-black text-slate-400 uppercase tracking-[0.2em] border-b border-slate-100"
          >
            {day}
          </div>
        ))}

        {/* Columna de Horas (Opcional visual)
            Aquí podrías renderizar etiquetas fijas para Morning1, Afternoon1, etc.
            en la primera columna para guiar al usuario.
        */}

        {/* Renderizado dinámico de las tarjetas */}
        {filteredEncounters.length > 0 ? (
          filteredEncounters.map((clase, idx) => (
            <EncounterCard key={`${clase.subject}-${idx}`} clase={clase} />
          ))
        ) : (
          <div class="col-span-7 py-20 text-center space-y-4">
            <div class="text-4xl">📅</div>
            <p class="text-slate-400 font-medium">No hay clases cargadas para {filter}. <br/> Sube un archivo PDF para comenzar.</p>
          </div>
        )}

        {/* Líneas de guía decorativas (opcional para mejorar la lectura de filas) */}
        <div class="absolute inset-0 grid grid-rows-9 pointer-events-none opacity-[0.03]">
          {Array.from({ length: 9 }).map((_, i) => (
            <div key={i} class="border-b border-slate-900 w-full h-full"></div>
          ))}
        </div>
      </div>
    </div>
  );
});