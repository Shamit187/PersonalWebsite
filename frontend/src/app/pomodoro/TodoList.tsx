"use client";

import { useState } from "react";
import { Activity } from "./types";

interface TodoListProps {
    todos: Activity[];
    setTodos: (todos: Activity[]) => void;
}

export default function TodoList({ todos, setTodos }: TodoListProps) {
    const [taskName, setTaskName] = useState("");
    const [taskTag, setTaskTag] = useState("");
    const [taskCounter, setTaskCounter] = useState(1);

    const addTask = () => {
        if (taskName.trim() === "") return;
        const newTask: Activity = {
            name: taskName,
            tag: taskTag || "General",
            counter: taskCounter
        };
        setTodos([...todos, newTask]);
        setTaskName("");
        setTaskTag("");
        setTaskCounter(1);
    };

    const deleteTask = (index: number) => {
        const updatedTodos = todos.filter((_, i) => i !== index);
        setTodos(updatedTodos);
    };

    const moveTaskUp = (index: number) => {
        if (index === 0) return;
        const updatedTodos = [...todos];
        [updatedTodos[index - 1], updatedTodos[index]] = [updatedTodos[index], updatedTodos[index - 1]];
        setTodos(updatedTodos);
    };

    const moveTaskDown = (index: number) => {
        if (index === todos.length - 1) return;
        const updatedTodos = [...todos];
        [updatedTodos[index], updatedTodos[index + 1]] = [updatedTodos[index + 1], updatedTodos[index]];
        setTodos(updatedTodos);
    };

    return (
        <div className="p-4 border rounded shadow-lg h-full flex flex-col">
            <h2 className="text-lg font-bold mb-2">Todo List</h2>
            <div className="flex gap-2 mb-4 overflow-x-auto">
                <input
                    type="text"
                    placeholder="Task name"
                    value={taskName}
                    onChange={(e) => setTaskName(e.target.value)}
                    className="border rounded w-full input-text"
                />
                <input
                    type="text"
                    placeholder="Tag (optional)"
                    value={taskTag}
                    onChange={(e) => setTaskTag(e.target.value)}
                    className="border rounded input-text"
                />
                <input
                    type="number"
                    min="1"
                    placeholder="Counter"
                    value={taskCounter}
                    onChange={(e) => setTaskCounter(Number(e.target.value))}
                    className="border rounded w-16 input-text"
                />
                <button
                    onClick={addTask}
                    className="px-4 py-2 bg-green-500 text-white rounded"
                >
                    Add
                </button>
            </div>

            {todos.length === 0 ? (
                <p className="text-gray-500">No tasks available.</p>
            ) : (
                <ul className="list-none">
                    {todos.map((todo, index) => (
                        <li
                            key={index}
                            className={`p-2 rounded ${index==0? "bg-gray-900" : "" } mb-2 flex justify-between items-center`}
                        >
                            <div>
                                <p className="font-semibold">{todo.name}</p>
                                <p className="text-sm text-gray-400">{todo.tag}</p>
                                <p className="text-sm">Counter: {todo.counter}</p>
                            </div>
                            <div className="flex gap-4">
                                <button
                                    onClick={() => moveTaskUp(index)}
                                    disabled={index === 0}
                                    className="activity-up"
                                >
                                    ↑
                                </button>
                                <button
                                    onClick={() => moveTaskDown(index)}
                                    disabled={index === todos.length - 1}
                                    className="activity-up"
                                >
                                    ↓
                                </button>
                                <button
                                    onClick={() => deleteTask(index)}
                                    className="activity-delete"
                                >
                                    ✖
                                </button>
                            </div>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
}
