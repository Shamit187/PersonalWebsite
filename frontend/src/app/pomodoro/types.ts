interface TimerValues {
    focusDuration: number;
    shortBreakDuration: number;
    longBreakDuration: number;
    spreelen: number;
}

interface TimeActivity {
    mode: "focus" | "shortBreak" | "longBreak";
    duration: number;
    timeLeft: number;
    spree: number;
    active: boolean;
}

interface Activity {
    tag: string;
    name: string;
    counter: number;
    id: number;
}

export type {TimerValues, TimeActivity, Activity};