import {JSX} from "solid-js";

/**
 * Simple password strength indicator.
 * - 0 ~ 20 is very dangerous (Red)
 * - 20 ~ 40 is dangerous (Red)
 * - 40 ~ 60 is very weak (Orange)
 * - 60 ~ 80 is weak (Yellow)
 * - 80 ~ 90 is good (Yellow)
 * - 90 ~ 95 is strong (Green)
 * - 95 ~ 99 is very strong (Green)
 * - 99 ~ 100 is invulnerable (Green)
 * @param strength - Function that returns the strength of the password.
 * @return {JSX.Element} - Span containing the password strength indicator.
 */
export default function PasswordStrengthIndicator({strength}: { strength: () => number }): JSX.Element {
    function color() {
        if (strength() < 40) {
            return 'bg-red-500';
        } else if (strength() < 60) {
            return 'bg-orange-500';
        } else if (strength() < 90) {
            return 'bg-yellow-500';
        } else {
            return 'bg-green-500';
        }
    }

    function inWords() {
        if (strength() === 0) {
            return 'Password strength';
        }
        else if (strength() < 20) {
            return 'Very dangerous';
        } else if (strength() < 40) {
            return 'Dangerous';
        } else if (strength() < 60) {
            return 'Very weak';
        } else if (strength() < 80) {
            return 'Weak';
        }
        else if (strength() < 90) {
            return 'Good';
        }
        else if (strength() < 95) {
            return 'Strong';
        }
        else if (strength() < 99) {
            return 'Very strong';
        }
        else {
            return 'Invulnerable';
        }
    }

    return (
        <div class="w-full flex flex-col justify-center">
            <span class="w-full h-1 rounded-full">
                <div class={`h-1 rounded-full ${color()} `} style={{ width: `${strength()}%` }}></div>
            </span>
            <div class="text-[14px] text-center">{inWords()}</div>
        </div>
    );
}