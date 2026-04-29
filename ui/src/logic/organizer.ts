    import type { Encounter } from './domain';

    export interface GroupOption {
        groupId: string;
        groupNumber: number;
        encounters: Encounter[];
    }

    export interface SubjectGroups {
        subject: string;
        options: GroupOption[];
    }

    /**
     * Transforma el arreglo plano de encuentros en una estructura jerárquica
     * de Asignatura -> Lista de Opciones de Grupo.
     */
    export function groupEncountersBySubject(encounters: Encounter[]): SubjectGroups[] {
        const subjectMap = new Map<string, Map<string, Encounter[]>>();

        for (const e of encounters) {
            if (!subjectMap.has(e.subject)) {
                subjectMap.set(e.subject, new Map());
            }
            const groupMap = subjectMap.get(e.subject)!;
            if (!groupMap.has(e.groupId)) {
                groupMap.set(e.groupId, []);
            }
            groupMap.get(e.groupId)!.push(e);
        }

        return Array.from(subjectMap.entries()).map(([subject, groupMap]) => ({
            subject,
            options: Array.from(groupMap.entries()).map(([groupId, groupEncounters]) => ({
                groupId,
                groupNumber: groupEncounters[0].group,
                encounters: groupEncounters
            })).sort((a, b) => a.groupNumber - b.groupNumber)
        })).sort((a, b) => a.subject.localeCompare(b.subject));
    }

    /**
     * Heurística de Primera Estimación:
     * Selecciona el Grupo 1 para cada asignatura encontrada para la carrera.
     */
    export function getDefaultSelectionIds(encounters: Encounter[]): Set<string> {
        const grouped = groupEncountersBySubject(encounters);
        const selection = new Set<string>();

        for (const subject of grouped) {
            // Buscamos preferentemente el Grupo 1 (G1)
            const g1 = subject.options.find(o => o.groupNumber === 1) || subject.options[0];
            if (g1) {
                selection.add(g1.groupId);
            }
        }

        return selection;
    }
