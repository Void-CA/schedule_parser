import { $, useStore } from "@builder.io/qwik";

// En app.tsx o un nuevo store de selección
const selectionStore = useStore<{ selectedIds: string[] }>({
    selectedIds: [] 
});

// Función para alternar selección de una clase
const toggleEncounter = $((id: string) => {
    if (selectionStore.selectedIds.includes(id)) {
        selectionStore.selectedIds = selectionStore.selectedIds.filter(i => i !== id);
    } else {
        selectionStore.selectedIds = [...selectionStore.selectedIds, id];
    }
});

export { selectionStore, toggleEncounter };