export type Day = 'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday';

export type AcademicBlock = 
    | 'Morning1' | 'Morning2' | 'Morning3' | 'Morning4'
    | 'Afternoon1' | 'Afternoon2' | 'Afternoon3' | 'Afternoon4';

export type Major = 'IGI' | 'IMS' | 'IME' | 'ICE' | 'IEE' | 'IEM' | 'LAF' | 'LCM';

export interface Encounter {
    majors_offered: Major[];
    subject: string;
    professor: string;
    room: string;
    day: Day;
    blocks: AcademicBlock[];
    group: number;
    // Frontend combinatory state
    uid?: string;
    isSelected?: boolean;
    isConflicted?: boolean;
}

export function hydrateEncounter(data: any): Encounter {
    if (!data || typeof data !== 'object') {
        throw new Error("Invalid encounter data: not an object");
    }
    
    // Vanilla TypeScript validation & defaults to prevent desynchronization
    return {
        majors_offered: Array.isArray(data.majors_offered) ? data.majors_offered : [],
        subject: String(data.subject || 'Unknown Subject'),
        professor: String(data.professor || 'Unknown Professor'),
        room: String(data.room || 'Unknown Room'),
        day: String(data.day) as Day, // Assert as from Rust
        blocks: Array.isArray(data.blocks) ? data.blocks : [],
        group: Number(data.group || 0),
    };
}
