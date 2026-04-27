import type { Encounter } from './domain';

export interface ConflictState {
    temporal: boolean;
    selection: boolean;
}

/**
 * Detecta conflictos entre un encuentro objetivo y un conjunto de encuentros ya seleccionados.
 * 
 * Reglas:
 * 1. Conflicto Temporal: El encuentro objetivo comparte día y al menos un bloque horario con otro encuentro seleccionado.
 * 2. Conflicto de Selección: El encuentro objetivo pertenece a una materia ya seleccionada pero a un grupo diferente.
 */
export function getConflictState(
    target: Encounter, 
    allEncounters: Encounter[],
    selectedGroupIds: Set<string>
): ConflictState {
    const state = { temporal: false, selection: false };
    
    // Obtenemos todos los encuentros que están actualmente seleccionados (excluyendo los del mismo grupo que el target si queremos evitar auto-conflicto, 
    // pero en realidad queremos saber si el target CHOCA con algo ya puesto)
    const activeSelectedEncounters = allEncounters.filter(e => 
        selectedGroupIds.has(e.groupId) && e.uid !== target.uid
    );

    for (const other of activeSelectedEncounters) {
        // 1. Conflicto de Selección (Misma materia, distinto grupo)
        if (target.subject === other.subject && target.groupId !== other.groupId) {
            state.selection = true;
        }

        // 2. Conflicto Temporal (Mismo día, solapamiento de bloques)
        if (target.day === other.day) {
            const hasOverlap = target.blocks.some(b => other.blocks.includes(b));
            if (hasOverlap) {
                state.temporal = true;
            }
        }
        
        // Si ya encontramos ambos tipos de conflicto, terminamos pronto
        if (state.temporal && state.selection) break;
    }
    
    return state;
}
