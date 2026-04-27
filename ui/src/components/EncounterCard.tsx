import { component$ } from '@builder.io/qwik';
import { getBlockRowStart, getDayColumn } from '../logic/utils.js';

export const EncounterCard = component$(({ clase }: { clase: any }) => {
    return (
        <div 
            style={{ 
                gridColumnStart: getDayColumn(clase.day), 
                gridRowStart: getBlockRowStart(clase.blocks),
                gridRowEnd: `span ${clase.blocks.length}`
            }}
            class="group relative flex flex-col justify-between p-3 rounded-2xl bg-blue-50/50 border border-blue-100 hover:bg-white hover:shadow-2xl hover:shadow-blue-200/50 transition-all duration-300 ease-out border-l-[6px] border-l-blue-500"
        >
            <div class="space-y-1">
                <div class="flex justify-between items-start gap-2">
                    <h3 class="text-[11px] font-black text-slate-800 leading-tight uppercase line-clamp-2">
                        {clase.subject}
                    </h3>
                    <span class="shrink-0 bg-blue-600 text-white text-[9px] px-2 py-0.5 rounded-lg font-bold shadow-sm">
                        G{clase.group}
                    </span>
                </div>
                <div class="inline-flex items-center gap-1.5 text-blue-700 bg-blue-100/50 px-2 py-0.5 rounded-md">
                    <span class="text-[10px] font-bold font-mono uppercase">{clase.room}</span>
                </div>
            </div>

            <div class="mt-2 pt-2 border-t border-blue-100/50">
                <p class="text-[9px] text-slate-400 font-medium truncate group-hover:text-slate-600 transition-colors">
                    {clase.professor}
                </p>
            </div>
        </div>
    );
});