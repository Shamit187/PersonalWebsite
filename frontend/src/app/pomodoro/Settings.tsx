"use client";

import { useState } from "react";
import { TimerValues, TimeActivity } from "./types";

interface SettingsProps {
    timerValues: TimerValues;
    setTimerValues: (values: TimerValues) => void;
    setTimeActivity: (activity: TimeActivity) => void;
}

export default function Settings({ timerValues, setTimerValues, setTimeActivity }: SettingsProps) {
    const [values, setValues] = useState<TimerValues>(timerValues);

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setValues(prev => ({ ...prev, [name]: parseInt(value) }));
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        setTimerValues(values);
        setTimeActivity({
            mode: "focus",
            duration: values.focusDuration,
            timeLeft: values.focusDuration * 60,
            spree: 0,
            active: false
        });
    };

    return (
        <div className="p-4 border rounded shadow-lg flex-grow flex flex-col justify-center">
            <h2 className="text-lg font-bold">Settings</h2>
            <form onSubmit={handleSubmit} className="mt-4 space-y-2">
                <div className="pomodoro-field">
                    <label className="block">Focus Duration:</label>
                    <input
                        type="number"
                        name="focusDuration"
                        value={values.focusDuration}
                        onChange={handleChange}
                        className="input-text"
                    />
                </div>
                <div className="pomodoro-field">
                    <label className="block">Short Break Duration:</label>
                    <input
                        type="number"
                        name="shortBreakDuration"
                        value={values.shortBreakDuration}
                        onChange={handleChange}
                        className="input-text"
                    />
                </div>
                <div className="pomodoro-field">
                    <label className="block">Long Break Duration:</label>
                    <input
                        type="number"
                        name="longBreakDuration"
                        value={values.longBreakDuration}
                        onChange={handleChange}
                        className="input-text"
                    />
                </div>
                <div className="pomodoro-field">
                    <label className="block">Spree Length:</label>
                    <input
                        type="number"
                        name="spreelen"
                        value={values.spreelen}
                        onChange={handleChange}
                        className="input-text"
                    />
                </div>
                <button type="submit" className="px-4 py-2 bg-blue-500 text-white rounded w-full">Save & Reset Timer</button>
            </form>
        </div>
    );
}
