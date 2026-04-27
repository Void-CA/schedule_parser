import { component$ } from '@builder.io/qwik';

interface MajorSelectorProps {
  selectedMajor: { value: string }; // Pasamos el signal de Qwik
}

const MAJORS = [
    { id: "ICE", name: "Ing. Cibernética Electrónica", icon: "🤖" },
    { id: "IMS", name: "Ing. Mecatrónica", icon: "⚙️" },
    { id: "IME", name: "Ing. Mecánica", icon: "🔧" },
    { id: "IGI", name: "Ing. Gestión Industrial", icon: "📊" },
    { id: "IEE", name: "Ing. Eléctrica", icon: "⚡" },
    { id: "IEM", name: "Ing. Electromédica", icon: "🏥" },
    { id: "LAF", name: "Lic. Administración Financiera", icon: "💰" },
    { id: "LCM", name: "Lic. Comercio y Mercadeo", icon: "📈" },
];

export const MajorSelector = component$(({ selectedMajor }: MajorSelectorProps) => {
  return (
    <div class="relative w-full lg:w-72 group">
      {/* Icono decorativo a la izquierda */}
      <div class="absolute left-4 top-1/2 -translate-y-1/2 pointer-events-none z-10 text-sm">
        {MAJORS.find(m => m.id === selectedMajor.value)?.icon || "🎓"}
      </div>

      <select 
        bind:value={selectedMajor}
        class="w-full pl-11 pr-10 py-3.5 bg-slate-50 border border-slate-200 text-slate-700 font-bold text-sm rounded-2xl 
               appearance-none cursor-pointer focus:ring-4 focus:ring-teal-100 focus:border-teal-400 focus:bg-white 
               outline-none transition-all duration-200 hover:border-slate-300"
      >
        {MAJORS.map((major) => (
          <option key={major.id} value={major.id} class="font-sans py-2">
            {major.name}
          </option>
        ))}
      </select>

      {/* Flecha personalizada a la derecha (Chevron) */}
      <div class="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-slate-400 group-hover:text-teal-500 transition-colors">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
          <path d="m6 9 6 6 6-6"/>
        </svg>
      </div>
    </div>
  );
});