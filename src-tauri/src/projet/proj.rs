use rand::Rng;

/// Calcule (x^n) % m en utilisant l'exponentiation rapide.
/// - `x`: Base.
/// - `n`: Exposant.
/// - `m`: Modulo.
/// Retourne le résultat de (x^n) % m.
/// Calculates (x^n) % m using fast exponentiation.
/// - `x`: Base.
/// - `n`: Exponent.
/// - `m`: Modulo.
/// Returns the result of (x^n) % m.
pub fn fast_expo(x: u64, n: u64, m: u64) -> u64 {
    let mut result: u128 = 1;
    let mut base: u128 = (x % m) as u128;
    let mut exp = n;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % (m as u128);
            // Check for potential overflow (optional, since u128 is large enough)
            if result > u64::MAX as u128 {
                panic!("Overflow detected in fast_expo");
            }
        }
        base = (base * base) % (m as u128);
        exp /= 2;
    }

    // Safely cast back to u64, since result % m is guaranteed to be less than m (u64)
    result as u64
}


/// Vérifie si un nombre `p` est probablement premier en utilisant
/// un test probabiliste avec les bases 2, 3, 5 et 7.
/// Retourne `true` si `p` est probablement premier, `false` sinon.
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
/// Retourne `gcd(a, b)`.
pub fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Vérifie si deux nombres entiers `a` et `b` sont relativement premiers.
/// Retourne `true` si `pgcd(a, b) == 1`, sinon `false`.
pub fn are_relatively_prime(a: u64, b: u64) -> bool {
    pgcd(a, b) == 1
}

/// Génère un nombre premier aléatoire compris entre 1 et `n`.
/// Si aucun nombre premier n'est trouvé après un certain nombre d'essais, retourne `None`.
pub fn generate_random_prime(n: u64) -> Option<u64> {
    if n < 2 {
        return None;
    }

    let mut rng = rand::thread_rng();
    const MAX_ATTEMPTS: u32 = 1000;

    for _ in 0..MAX_ATTEMPTS {
        let candidate = rng.gen::<u64>() % (n - 1) + 2; // Génère un nombre entre 2 et `n` inclus
        if is_probably_prime(candidate) {
            return Some(candidate);
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_expo() {
        // Test de base
        assert_eq!(fast_expo(2, 3, 5), 3); // 2^3 % 5 = 8 % 5 = 3

        // Test avec exponentiation par zéro
        assert_eq!(fast_expo(10, 0, 7), 1); // 10^0 % 7 = 1

        // Test avec m = 1
        assert_eq!(fast_expo(10, 5, 1), 0); // Tout modulo 1 est 0

        // Test avec grands nombres
        assert_eq!(
            fast_expo(123456789, 12345, 1000000007),
            fast_expo_brute(123456789, 12345, 1000000007)
        );
    }

    // Fonction auxiliaire pour vérifier fast_expo avec une méthode brute
    fn fast_expo_brute(x: u64, n: u64, m: u64) -> u64 {
        let mut result = 1;
        for _ in 0..n {
            result = (result * x) % m;
        }
        result
    }

    #[test]
    fn test_is_probably_prime() {
        // Nombres premiers connus
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 997, 1009, 104729];
        for &p in &primes {
            assert!(is_probably_prime(p), "{} devrait être premier", p);
        }

        // Nombres non premiers
        let non_primes = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 1000, 1001, 104728];
        for &np in &non_primes {
            assert!(!is_probably_prime(np), "{} ne devrait pas être premier", np);
        }
    }

    #[test]
    fn test_pgcd() {
        // Cas de base
        assert_eq!(pgcd(54, 24), 6);
        assert_eq!(pgcd(48, 18), 6);
        assert_eq!(pgcd(101, 10), 1);

        // Cas où l'un des nombres est zéro
        assert_eq!(pgcd(0, 5), 5);
        assert_eq!(pgcd(5, 0), 5);
        assert_eq!(pgcd(0, 0), 0);

        // Cas avec deux nombres égaux
        assert_eq!(pgcd(7, 7), 7);
        assert_eq!(pgcd(100, 100), 100);
    }

    #[test]
    fn test_are_relatively_prime() {
        // Pairs relativement premiers
        assert!(are_relatively_prime(14, 15));
        assert!(are_relatively_prime(17, 31));
        assert!(are_relatively_prime(1, 100));
        assert!(are_relatively_prime(13, 27));

        // Pairs non relativement premiers
        assert!(!are_relatively_prime(14, 21));
        assert!(!are_relatively_prime(100, 10));
        assert!(!are_relatively_prime(12, 18));
        assert!(!are_relatively_prime(0, 5));
        assert!(!are_relatively_prime(0, 0));
    }

    #[test]
    fn test_generate_random_prime() {
        // Générer des petits nombres premiers
        if let Some(p) = generate_random_prime(30) {
            assert!(p <= 30);
            assert!(is_probably_prime(p));
        } else {
            panic!("Aucun nombre premier trouvé dans la plage spécifiée.");
        }

        // Générer un nombre premier dans une plage plus grande
        if let Some(p) = generate_random_prime(20000000000) {
            assert!(p <= 20000000000);
            assert!(is_probably_prime(p));
        } else {
            panic!("Aucun nombre premier trouvé dans la plage spécifiée.");
        }

        // Tester avec n < 2 (aucun nombre premier possible)
        assert_eq!(generate_random_prime(1), None);
    }
}
