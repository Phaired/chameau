use rand::Rng;

/// Calcule (x^n) % m en utilisant l'exponentiation rapide.
pub fn fast_expo(x: u64, n: u64, m: u64) -> u64 {
    let mut result: u128 = 1;
    let mut base: u128 = (x % m) as u128;
    let mut exp = n;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % (m as u128);
        }
        base = (base * base) % (m as u128);
        exp /= 2;
    }
    result as u64
}

/// Vérifie si un nombre `p` est probablement premier en utilisant
/// un test probabiliste avec les bases 2, 3, 5 et 7.
pub fn is_probably_prime(p: u64) -> bool {
    if p < 2 {
        return false;
    }
    let bases = [2, 3, 5, 7];
    for &base in &bases {
        if base >= p {
            continue;
        }
        if fast_expo(base, p - 1, p) != 1 {
            return false;
        }
    }
    true
}

/// Calcule le plus grand diviseur commun (GCD) de deux nombres entiers `a` et `b`
/// en utilisant l'algorithme d'Euclide.
pub fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Vérifie si deux nombres entiers `a` et `b` sont relativement premiers.
pub fn are_relatively_prime(a: u64, b: u64) -> bool {
    pgcd(a, b) == 1
}

/// Génère un nombre premier aléatoire compris entre 2 et `n` inclus.
/// Retourne `Some(prime)` si un nombre premier est trouvé, ou `None` sinon.
pub fn generate_random_prime(n: u64) -> Option<u64> {
    if n < 2 {
        return None;
    }
    let mut rng = rand::thread_rng();
    const MAX_ATTEMPTS: u32 = 1000;
    for _ in 0..MAX_ATTEMPTS {
        // Génère un entier entre 2 et n inclus
        let candidate = rng.gen_range(2, n + 1);
        if is_probably_prime(candidate) {
            return Some(candidate);
        }
    }
    None
}

/// Recherche par force brute l'inverse modulaire de `e` modulo `phi`.
/// C'est-à-dire, trouve d tel que (e * d) % phi == 1.
pub fn mod_inverse(e: u64, phi: u64) -> Option<u64> {
    for d in 1..phi {
        if (e as u128 * d as u128) % (phi as u128) == 1 {
            return Some(d);
        }
    }
    None
}

/// Génère une paire de clés RSA.
/// La procédure est la suivante:
/// 1. Génération aléatoire de deux nombres premiers distincts `p` et `q`
///    (dans la plage [2, max]).
/// 2. Calcul de n = p * q et de φ(n) = (p - 1)*(q - 1)
/// 3. Choix de e tel que 1 < e < φ(n) et e est premier avec φ(n)
/// 4. Calcul de d l'inverse modulaire de e modulo φ(n)
///
/// Retourne un tuple ((n, e), (n, d)) où (n, e) est la clé publique
/// et (n, d) la clé privée.
pub fn generate_rsa_keys(max: u64) -> Option<((u64, u64), (u64, u64))> {
    // Génère deux nombres premiers distincts p et q
    let p = generate_random_prime(max)?;
    let mut q = generate_random_prime(max)?;
    while q == p {
        q = generate_random_prime(max)?;
    }
    let n = p.checked_mul(q)?;
    let phi = (p - 1).checked_mul(q - 1)?;

    // Choix de e: on tente d'utiliser la valeur classique 65537 si possible.
    let e = if phi > 65537 && are_relatively_prime(65537, phi) {
        65537
    } else {
        let mut rng = rand::thread_rng();
        loop {
            let candidate = rng.gen_range(2, phi);
            if are_relatively_prime(candidate, phi) {
                break candidate;
            }
        }
    };

    let d = mod_inverse(e, phi)?;
    Some(((n, e), (n, d)))
}

/// Signe un message `M` (représenté par un entier) en utilisant la clé privée (n, d).
/// Le calcul effectué est S = M^d mod n.
pub fn sign_message(message: u64, private_key: (u64, u64)) -> u64 {
    let (n, d) = private_key;
    fast_expo(message, d, n)
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test de l'exponentiation rapide.
    #[test]
    fn test_fast_expo() {
        // Cas simple : 2^3 % 5 = 8 % 5 = 3.
        assert_eq!(fast_expo(2, 3, 5), 3);
        // Tout nombre élevé à la puissance 0 doit donner 1.
        assert_eq!(fast_expo(10, 0, 7), 1);
        // Test avec modulo égal à 1, le résultat doit toujours être 0.
        assert_eq!(fast_expo(10, 5, 1), 0);
        // Comparaison avec une méthode brute pour des valeurs importantes.
        assert_eq!(
            fast_expo(123456789, 12345, 1000000007),
            fast_expo_brute(123456789, 12345, 1000000007)
        );
    }

    // Méthode brute pour vérifier l'exactitude de fast_expo.
    fn fast_expo_brute(x: u64, n: u64, m: u64) -> u64 {
        let mut result = 1;
        for _ in 0..n {
            result = (result * x) % m;
        }
        result
    }

    // Test de la fonction de primalité probabiliste.
    #[test]
    fn test_is_probably_prime() {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 997, 1009, 104729];
        // Pour chaque nombre premier attendu, la fonction doit retourner true.
        for &p in &primes {
            assert!(is_probably_prime(p), "{} devrait être premier", p);
        }
        let non_primes = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 1000, 1001, 104728];
        // Pour chaque nombre non premier, la fonction doit retourner false.
        for &np in &non_primes {
            assert!(!is_probably_prime(np), "{} ne devrait pas être premier", np);
        }
    }

    // Test de l'algorithme de calcul du PGCD (plus grand commun diviseur).
    #[test]
    fn test_pgcd() {
        // PGCD de 54 et 24 doit être 6.
        assert_eq!(pgcd(54, 24), 6);
        // PGCD de 48 et 18 doit être 6.
        assert_eq!(pgcd(48, 18), 6);
        // 101 et 10 sont premiers entre eux, donc PGCD = 1.
        assert_eq!(pgcd(101, 10), 1);
        // Cas avec un des paramètres à zéro.
        assert_eq!(pgcd(0, 5), 5);
        assert_eq!(pgcd(5, 0), 5);
        // Cas particulier où les deux nombres sont zéro.
        assert_eq!(pgcd(0, 0), 0);
        // PGCD de deux mêmes nombres.
        assert_eq!(pgcd(7, 7), 7);
        assert_eq!(pgcd(100, 100), 100);
    }

    // Test pour vérifier si deux nombres sont relativement premiers.
    #[test]
    fn test_are_relatively_prime() {
        // Ces paires doivent être relativement premières.
        assert!(are_relatively_prime(14, 15));
        assert!(are_relatively_prime(17, 31));
        assert!(are_relatively_prime(1, 100));
        assert!(are_relatively_prime(13, 27));
        // Ces paires ne le sont pas.
        assert!(!are_relatively_prime(14, 21));
        assert!(!are_relatively_prime(100, 10));
        assert!(!are_relatively_prime(12, 18));
        // Cas avec zéro.
        assert!(!are_relatively_prime(0, 5));
        assert!(!are_relatively_prime(0, 0));
    }

    // Test de la génération aléatoire d'un nombre premier dans un intervalle donné.
    #[test]
    fn test_generate_random_prime() {
        // Test avec une borne faible.
        if let Some(p) = generate_random_prime(30) {
            assert!(p <= 30);
            assert!(is_probably_prime(p));
        } else {
            panic!("Aucun nombre premier trouvé dans la plage spécifiée (max = 30).");
        }
        // Test avec une borne plus élevée.
        if let Some(p) = generate_random_prime(20000000000) {
            assert!(p <= 20000000000);
            assert!(is_probably_prime(p));
        } else {
            panic!("Aucun nombre premier trouvé dans la plage spécifiée (max = 20000000000).");
        }
        // Aucun nombre premier ne peut être généré pour n < 2.
        assert_eq!(generate_random_prime(1), None);
    }

    // Test de la fonction de calcul de l'inverse modulaire.
    #[test]
    fn test_mod_inverse() {
        // Exemple : pour e = 7 et φ = 40, d doit être 23 car 7 * 23 = 161 ≡ 1 mod 40.
        assert_eq!(mod_inverse(7, 40), Some(23));
        // Exemple : pour e = 3 et φ = 10, d doit être 7 car 3 * 7 = 21 ≡ 1 mod 10.
        assert_eq!(mod_inverse(3, 10), Some(7));
    }

    // Test pour vérifier qu'aucun inverse n'existe dans certains cas.
    #[test]
    fn test_mod_inverse_failure() {
        // Pour e = 2 et φ = 4, aucun inverse modulaire n'existe car 2 et 4 ne sont pas inversibles.
        assert_eq!(mod_inverse(2, 4), None);
    }

    // Test complet du processus RSA : génération de clés, signature et vérification.
    #[test]
    fn test_rsa_signature() {
        // Génération des clés RSA avec une borne raisonnable pour garantir que le message soit inférieur à n.
        let keys = generate_rsa_keys(1000).expect("Échec de la génération des clés RSA");
        let ((n, e), (_, d)) = keys;

        // Choix d'un message à signer (doit être inférieur à n).
        let message: u64 = 42;
        assert!(message < n, "Le message doit être inférieur à n pour RSA");

        // Signature du message avec la clé privée (n, d).
        let signature = sign_message(message, (n, d));

        // Vérification de la signature via l'opération S^e mod n.
        let decoded_message = fast_expo(signature, e, n);

        // La signature est valide si le message décodé correspond au message original modulo n.
        assert_eq!(decoded_message, message % n, "La vérification de la signature a échoué");
    }

    // Test supplémentaire pour vérifier la signature lorsque le message est 0.
    #[test]
    fn test_sign_message_zero() {
        // Génération des clés RSA.
        let keys = generate_rsa_keys(1000).expect("Échec de la génération des clés RSA");
        let ((n, e), (_, d)) = keys;

        // Cas particulier : le message 0 doit rester 0 après signature.
        let message: u64 = 0;
        assert!(message < n, "Le message doit être inférieur à n pour RSA");

        let signature = sign_message(message, (n, d));
        let decoded_message = fast_expo(signature, e, n);

        // Vérification : 0 signé doit redonner 0.
        assert_eq!(decoded_message, 0, "La signature du message 0 a échoué");
    }
}