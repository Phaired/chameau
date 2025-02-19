// App.tsx (extrait)
import './App.css';
import { Button } from "@/components/ui/button.tsx";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input.tsx";

const App = () => {
    const [bigPrime, setBigPrime] = useState<number | null>(null);
    const [primeError, setPrimeError] = useState<string | null>(null);
    const [rsaKeys, setRSAKeys] = useState<[[number, number], [number, number]] | null>(null);
    const [rsaError, setRSAError] = useState<string | null>(null);
    const [message, setMessage] = useState<number>(0);
    const [privateN, setPrivateN] = useState<number>(0);
    const [privateD, setPrivateD] = useState<number>(0);
    const [signature, setSignature] = useState<number | null>(null);
    const [signError, setSignError] = useState<string | null>(null);
    const [verifySignature, setVerifySignature] = useState<number>(0);
    const [publicN, setPublicN] = useState<number>(0);
    const [publicE, setPublicE] = useState<number>(0);
    const [decoded, setDecoded] = useState<number | null>(null);

    const handleGenerateBigPrime = async () => {
        try {
            const prime: number = await invoke("generate_big_prime", { max: 1844674407370955 });
            setBigPrime(prime);
            setPrimeError(null);
        } catch (error) {
            setBigPrime(null);
            setPrimeError("Échec lors de la génération du grand nombre premier.");
        }
    };

    const handleGenerateRSAKeys = async () => {
        try {
            const keys: [[number, number], [number, number]] = await invoke("generate_rsa_keys_command", { max: 10000 });
            setRSAKeys(keys);
            setRSAError(null);
        } catch (error) {
            setRSAKeys(null);
            setRSAError("Échec lors de la génération des clés RSA.");
        }
    };

    const handleSignMessage = async () => {
        try {
            const sig: number = await invoke("sign_message_command", { message, privateN, privateD });
            setSignature(sig);
            setSignError(null);
        } catch (error) {
            setSignature(null);
            setSignError("Échec lors de la signature du message.");
        }
    };

    const handleDecodeMessage = async () => {
        try {
            const dec: number = await invoke("decode_message_command", { signature: verifySignature, publicN, publicE });
            setDecoded(dec);
        } catch (error) {
            setDecoded(null);
        }
    };

    return (
        <div className="App" style={{ padding: "2rem" }}>
            <section style={{ marginBottom: "2rem" }}>
                <h2>Générer un grand nombre premier</h2>
                <Button variant="outline" onClick={handleGenerateBigPrime}>Générer un grand nombre premier</Button>
                {bigPrime !== null && <p>Nombre premier : {bigPrime}</p>}
                {primeError && <p className="text-red-500">{primeError}</p>}
            </section>

            <section style={{ marginBottom: "2rem" }}>
                <h2>Générer des clés RSA</h2>
                <Button variant="outline" onClick={handleGenerateRSAKeys}>Générer des clés RSA</Button>
                {rsaKeys && (
                    <div>
                        <p><strong>Clé publique (n, e):</strong> {rsaKeys[0][0]}, {rsaKeys[0][1]}</p>
                        <p><strong>Clé privée (n, d):</strong> {rsaKeys[1][0]}, {rsaKeys[1][1]}</p>
                    </div>
                )}
                {rsaError && <p className="text-red-500">{rsaError}</p>}
            </section>

            <section style={{ marginBottom: "2rem" }}>
                <h2>Signer le message</h2>
                <div>
                    <Input type="number" placeholder="Message" onChange={(e) => setMessage(Number(e.target.value))} />
                    <Input type="number" placeholder="Modulus (n)" onChange={(e) => setPrivateN(Number(e.target.value))} />
                    <Input type="number" placeholder="Exponent (d)" onChange={(e) => setPrivateD(Number(e.target.value))} />
                </div>
                <Button variant="outline" onClick={handleSignMessage}>Signer le message</Button>
                {signature !== null && <p>Signature : {signature}</p>}
                {signError && <p className="text-red-500">{signError}</p>}
            </section>

            <section style={{ marginBottom: "2rem" }}>
                <h2>Décoder et vérifier la signature</h2>
                <div>
                    <Input type="number" placeholder="Signature" onChange={(e) => setVerifySignature(Number(e.target.value))} />
                    <Input type="number" placeholder="Modulus (n)" onChange={(e) => setPublicN(Number(e.target.value))} />
                    <Input type="number" placeholder="Exponent (e)" onChange={(e) => setPublicE(Number(e.target.value))} />
                </div>
                <div style={{ marginTop: "1rem" }}>
                    <Button variant="outline" onClick={handleDecodeMessage}>Décoder le message</Button>
                    {decoded !== null && <p>Message décodé : {decoded}</p>}
                </div>
            </section>
        </div>
    );
};

export default App;
