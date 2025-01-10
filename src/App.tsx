import { Button } from "@/components/ui/button.tsx";
import { useState } from "react";
import {invoke} from "@tauri-apps/api/core";

const App = () => {
    const [bigPrime, setBigPrime] = useState<number | null>(null);

    const handleGenerateBigPrime = async () => {
        try {
            const prime: number = await invoke("generate_big_prime", { max: 100000 });
            setBigPrime(prime);
        } catch (error) {
            console.error("Failed to generate big prime:", error);
            setBigPrime(null);
        }
    };

    return (
        <>
            <Button onClick={handleGenerateBigPrime}>
                Generate big prime number
            </Button>
            {bigPrime && <p>Big prime: {bigPrime}</p>}
        </>
    );
};

export default App;
