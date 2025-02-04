"use client";

import { useState, useEffect } from "react";
import { TimeActivity, TimerValues, Activity } from "./types";

interface TimerProps {
    timerValues: TimerValues;
    timeActivity: TimeActivity;
    setTimeActivity: (activity: TimeActivity) => void;
    todos: Activity[];
    setTodos: (todos: Activity[]) => void;
}


export default function Timer({ timerValues, timeActivity, setTimeActivity, todos, setTodos }: TimerProps) {
    const [startTime, setStartTime] = useState<number | null>(null);
    const [elapsedTime, setElapsedTime] = useState(0);

    useEffect(() => {
        let timer: NodeJS.Timeout;
        if (timeActivity.active) {
            if (startTime === null) {
                setStartTime(Date.now());
            }
            timer = setInterval(() => {
                const now = Date.now();
                const elapsed = Math.floor((now - (startTime || now)) / 1000);
                const newTimeLeft = Math.max(timeActivity.duration * 60 - elapsed, 0);
                setElapsedTime(elapsed);
                setTimeActivity({ ...timeActivity, timeLeft: newTimeLeft });

                if (newTimeLeft === 0) {
                    handleSessionEnd();
                }
            }, 1000);
        }
        return () => clearInterval(timer);
    }, [timeActivity.active, startTime]);

    const handleSessionEnd = () => {
        let updatedTodos = [...todos];
    
        if (timeActivity.mode === "focus") {
            // Reduce counter of active task
            if (updatedTodos.length > 0) {
                updatedTodos[0].counter -= 1;
    
                // Remove task if counter reaches 0
                if (updatedTodos[0].counter === 0) {
                    updatedTodos.shift(); // Remove first task
                }
            }
    
            // Switch to break mode after completing focus
            const newSpree = timeActivity.spree + 1;
            const nextMode = newSpree % timerValues.spreelen === 0 ? "longBreak" : "shortBreak";
            const nextDuration = nextMode === "longBreak" ? timerValues.longBreakDuration : timerValues.shortBreakDuration;
    
            setTimeActivity({
                mode: nextMode,
                duration: nextDuration,
                timeLeft: nextDuration * 60,
                spree: newSpree,
                active: false
            });
        } else {
            // Return to focus mode after a break
            setTimeActivity({
                mode: "focus",
                duration: timerValues.focusDuration,
                timeLeft: timerValues.focusDuration * 60,
                spree: timeActivity.spree,
                active: false
            });
        }
    
        // Update the task list
        setTodos(updatedTodos);
        setStartTime(null);
        setElapsedTime(0);
    };
    

    const toggleTimer = () => {
        setTimeActivity({ ...timeActivity, active: !timeActivity.active });
        if (!timeActivity.active) {
            setStartTime(Date.now() - elapsedTime * 1000);
        }
    };

    const resetTimer = () => {
        setTimeActivity({
            ...timeActivity,
            timeLeft: timeActivity.duration * 60,
            active: false
        });
        setStartTime(null);
        setElapsedTime(0);
    };

    const formatTime = (seconds: number) => {
        const minutes = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${minutes}:${secs < 10 ? "0" : ""}${secs}`;
    };

    return (
        <div className="h-80 p-4 border rounded shadow-lg flex flex-col justify-center">
            <div className="text-center">
                <h2 className="text-lg font-bold">
                    {timeActivity.mode === "focus"
                        ? "Focus Time"
                        : timeActivity.mode === "shortBreak"
                        ? "Short Break"
                        : "Long Break"}
                </h2>
                <p className="text-2xl font-mono mt-2">{formatTime(timeActivity.timeLeft)}</p>
                <div className="mt-4 flex justify-center gap-4">
                    <button onClick={toggleTimer} className="px-4 py-2 bg-blue-500 text-white rounded">
                        {timeActivity.active ? "Pause" : "Start"}
                    </button>
                    <button onClick={resetTimer} className="px-4 py-2 bg-gray-500 text-white rounded">
                        Reset
                    </button>
                </div>
                <div className="mt-4">
                    <p>Spree: {timeActivity.spree}</p>
                    <p>Active Tasks: {todos.length}</p>
                </div>
            </div>
        </div>
    );
}
