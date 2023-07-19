import { writable } from 'svelte/store';

interface NavLink {
    linek: string,
    icon: HTMLElement
}
function createFooterNav() {
    const {subscribe, set, update} = writable([""])

    return {
        subscribe,
        update: (a: NavLink[]) => update(() => [...a]),
        rest: () => set([])
    }
}

export const footerNav = createFooterNav()