// src/App.tsx
import './App.css';

import { Button } from "@/components/ui/button.tsx";
import { useState } from "react";
import {invoke} from "@tauri-apps/api/core";

const App = () => {
    const [bigPrime, setBigPrime] = useState<number | null>(null);
    const [error, setError] = useState<string | null>(null); // New state for errors

    const handleGenerateBigPrime = async () => {
        try {
            const prime: number = await invoke("generate_big_prime", { max: 1844674407370955 });
            setBigPrime(prime);
            setError(null); // Clear any previous errors
        } catch (error) {
            console.error("Failed to generate big prime:", error);
            setBigPrime(null);
            setError("Failed to generate a big prime number.");
        }
    };

    return (
        <>
            <Button variant="outline" onClick={handleGenerateBigPrime}>
                Generate big prime number
            </Button>
            {bigPrime && <p>Big prime: {bigPrime}</p>}
            {error && <p className="text-red-500">{error}</p>} {/* Display error */}
        </>
    );
};

export default App;
