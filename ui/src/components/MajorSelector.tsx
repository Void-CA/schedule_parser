interface MajorSelectorProps {
  selectedMajor: string;
  setSelectedMajor: (val: string) => void;
}

const MAJORS = [
    { id: "ICE", name: "Ing. Cibernética Electrónica" },
    { id: "IMS", name: "Ing. Mecatrónica" },
    { id: "IME", name: "Ing. Mecánica" },
    { id: "IGI", name: "Ing. Gestión Industrial" },
    { id: "IEE", name: "Ing. Eléctrica" },
    { id: "IEM", name: "Ing. Electromédica" },
    { id: "LAF", name: "Lic. Administración Financiera" },
    { id: "LCM", name: "Lic. Comercio y Mercadeo" },
];

export const MajorSelector = ({ selectedMajor, setSelectedMajor }: MajorSelectorProps) => {
  return (
    <div className="relative w-full lg:w-72 group">
      <select 
        value={selectedMajor}
        onChange={(e) => setSelectedMajor(e.target.value)}
        className="w-full pl-4 pr-10 py-3.5 bg-slate-50 border border-slate-200 text-slate-700 font-bold text-sm rounded-2xl 
               appearance-none cursor-pointer focus:ring-4 focus:ring-teal-100 focus:border-teal-400 focus:bg-white 
               outline-none transition-all duration-200 hover:border-slate-300"
      >
        {MAJORS.map((major) => (
          <option key={major.id} value={major.id} className="font-sans py-2">
            {major.name}
          </option>
        ))}
      </select>

      <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-slate-400 group-hover:text-teal-500 transition-colors">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round">
          <path d="m6 9 6 6 6-6"/>
        </svg>
      </div>
    </div>
  );
};