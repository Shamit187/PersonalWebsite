"use client";

import { useState, useEffect } from "react";
import { Activity, TimeActivity, TimerValues } from "./types";
import Timer from "./Timer";
import Settings from "./Settings";
import TodoList from "./TodoList";


export default function Home() {
    // managing settings
    const [timerValues, setTimerValues] = useState<TimerValues>({
        focusDuration: 25,
        shortBreakDuration: 5,
        longBreakDuration: 15,
        spreelen: 4
    });

    // managing time activity
    const [timeActivity, setTimeActivity] = useState<TimeActivity>({
        mode: "focus",
        duration: 25,
        timeLeft: 25 * 60,
        spree: 0,
        active: false
    });

    // managing todo list
    const [todos, setTodos] = useState<Activity[]>([]);

    return (

        <div className="grid grid-cols-6 gap-4 w-screen h-screen px-4 py-8">
            <div className="col-span-2 flex flex-col gap-4">
                <Timer timerValues={timerValues} timeActivity={timeActivity} setTimeActivity={setTimeActivity} todos={todos} setTodos={setTodos} />
                <Settings timerValues={timerValues} setTimerValues={setTimerValues} setTimeActivity={setTimeActivity} />
            </div>
            <div className="col-span-4">
                <TodoList todos={todos} setTodos={setTodos} />
            </div>
        </div>

    );
}