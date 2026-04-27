const getDayColumn = (day: string): number => {
    const map: Record<string, number> = {
        "Monday": 2, "Tuesday": 3, "Wednesday": 4, "Thursday": 5, "Friday": 6, "Saturday": 7
    };
    return map[day] || 2;
};

const getBlockRowStart = (blocks: string[]): number => {
    const map: Record<string, number> = {
        "Morning1": 2, "Morning2": 3, "Morning3": 4, "Morning4": 5, 
        "Afternoon1": 6, "Afternoon2": 7, "Afternoon3": 8, "Afternoon4": 9, 
    };
    return map[blocks[0]] || 2;
};

export { getDayColumn, getBlockRowStart };