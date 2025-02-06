// src/App.tsx
import './App.css';
import { Button } from "@/components/ui/button.tsx";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const App = () => {
    // État pour la génération d'un grand nombre premier
    const [bigPrime, setBigPrime] = useState<number | null>(null);
    const [primeError, setPrimeError] = useState<string | null>(null);

    // État pour la génération des clés RSA
    const [rsaKeys, setRSAKeys] = useState<[[number, number], [number, number]] | null>(null);
    const [rsaError, setRSAError] = useState<string | null>(null);

    // États pour la signature d'un message
    const [message, setMessage] = useState<number>(0);
    const [privateN, setPrivateN] = useState<number>(0);
    const [privateD, setPrivateD] = useState<number>(0);
    const [signature, setSignature] = useState<number | null>(null);
    const [signError, setSignError] = useState<string | null>(null);

    // États pour le décodage et la vérification d'une signature
    const [verifyMessage, setVerifyMessage] = useState<number>(0);
    const [verifySignature, setVerifySignature] = useState<number>(0);
    const [publicN, setPublicN] = useState<number>(0);
    const [publicE, setPublicE] = useState<number>(0);
    const [decoded, setDecoded] = useState<number | null>(null);
    const [verifyResult, setVerifyResult] = useState<boolean | null>(null);
    const [verifyError, setVerifyError] = useState<string | null>(null);

    // Commande : Générer un grand nombre premier
    const handleGenerateBigPrime = async () => {
        try {
            const prime: number = await invoke("generate_big_prime", { max: 1844674407370955 });
            setBigPrime(prime);
            setPrimeError(null);
        } catch (error) {
            console.error("Failed to generate big prime:", error);
            setBigPrime(null);
            setPrimeError("Failed to generate a big prime number.");
        }
    };

    // Commande : Générer les clés RSA
    const handleGenerateRSAKeys = async () => {
        try {
            // Ici, nous utilisons 1000 comme borne max pour la génération des nombres premiers.
            const keys: [[number, number], [number, number]] = await invoke("generate_rsa_keys_command", { max: 10000 });
            setRSAKeys(keys);
            setRSAError(null);
        } catch (error) {
            console.error("Failed to generate RSA keys:", error);
            setRSAKeys(null);
            setRSAError("Failed to generate RSA keys.");
        }
    };

    // Commande : Signer un message avec la clé privée (n, d)
    const handleSignMessage = async () => {
        try {
            const sig: number = await invoke("sign_message_command", {
                message,
                privateN, // attention à la casse attendue par Rust
                privateD
            });
            setSignature(sig);
            setSignError(null);
        } catch (error) {
            console.error("Failed to sign message:", error);
            setSignature(null);
            setSignError("Failed to sign message.");
        }
    };

    // Commande : Décoder le message signé à l'aide de la clé publique (n, e)
    const handleDecodeMessage = async () => {
        try {
            const dec: number = await invoke("decode_message_command", {
                signature: verifySignature,
                publicN,
                publicE,
            });
            setDecoded(dec);
        } catch (error) {
            console.error("Failed to decode message:", error);
            setDecoded(null);
        }
    };

    // Commande : Vérifier une signature avec la clé publique (n, e)
    const handleVerifySignature = async () => {
        try {
            const result: boolean = await invoke("verify_signature_command", {
                message: verifyMessage,
                signature: verifySignature,
                publicN,
                publicE,
            });
            setVerifyResult(result);
            setVerifyError(null);
        } catch (error) {
            console.error("Failed to verify signature:", error);
            setVerifyResult(null);
            setVerifyError("Failed to verify signature.");
        }
    };

    return (
        <div className="App" style={{ padding: "2rem" }}>
            {/* Génération d'un grand nombre premier */}
            <section style={{ marginBottom: "2rem" }}>
                <h2>Generate Big Prime Number</h2>
                <Button variant="outline" onClick={handleGenerateBigPrime}>
                    Generate big prime number
                </Button>
                {bigPrime !== null && <p>Big prime: {bigPrime}</p>}
                {primeError && <p className="text-red-500">{primeError}</p>}
            </section>

            {/* Génération des clés RSA */}
            <section style={{ marginBottom: "2rem" }}>
                <h2>Generate RSA Keys</h2>
                <Button variant="outline" onClick={handleGenerateRSAKeys}>
                    Generate RSA Keys
                </Button>
                {rsaKeys && (
                    <div>
                        <p>
                            <strong>Public Key (n, e):</strong> {rsaKeys[0][0]}, {rsaKeys[0][1]}
                        </p>
                        <p>
                            <strong>Private Key (n, d):</strong> {rsaKeys[1][0]}, {rsaKeys[1][1]}
                        </p>
                    </div>
                )}
                {rsaError && <p className="text-red-500">{rsaError}</p>}
            </section>

            {/* Signature d'un message */}
            <section style={{ marginBottom: "2rem" }}>
                <h2>Sign Message</h2>
                <div>
                    <input
                        type="number"
                        placeholder="Message"
                        onChange={(e) => setMessage(Number(e.target.value))}
                    />
                    <input
                        type="number"
                        placeholder="Private n"
                        onChange={(e) => setPrivateN(Number(e.target.value))}
                    />
                    <input
                        type="number"
                        placeholder="Private d"
                        onChange={(e) => setPrivateD(Number(e.target.value))}
                    />
                </div>
                <Button variant="outline" onClick={handleSignMessage}>
                    Sign Message
                </Button>
                {signature !== null && <p>Signature: {signature}</p>}
                {signError && <p className="text-red-500">{signError}</p>}
            </section>

            {/* Décodage et vérification d'une signature */}
            <section style={{ marginBottom: "2rem" }}>
                <h2>Decode Message and Verify Signature</h2>
                <div>
                    <input
                        type="number"
                        placeholder="Message"
                        onChange={(e) => setVerifyMessage(Number(e.target.value))}
                    />
                    <input
                        type="number"
                        placeholder="Signature"
                        onChange={(e) => setVerifySignature(Number(e.target.value))}
                    />
                    <input
                        type="number"
                        placeholder="Public n"
                        onChange={(e) => setPublicN(Number(e.target.value))}
                    />
                    <input
                        type="number"
                        placeholder="Public e"
                        onChange={(e) => setPublicE(Number(e.target.value))}
                    />
                </div>
                <div style={{ marginTop: "1rem" }}>
                    <Button variant="outline" onClick={handleDecodeMessage}>
                        Decode Message
                    </Button>
                    {decoded !== null && <p>Decoded Message: {decoded}</p>}
                </div>
                <div style={{ marginTop: "1rem" }}>
                    <Button variant="outline" onClick={handleVerifySignature}>
                        Verify Signature
                    </Button>
                    {verifyResult !== null && (
                        <p>
                            Verification result:{" "}
                            {verifyResult ? "Valid Signature" : "Invalid Signature"}
                        </p>
                    )}
                    {verifyError && <p className="text-red-500">{verifyError}</p>}
                </div>
            </section>
        </div>
    );
};

export default App;
