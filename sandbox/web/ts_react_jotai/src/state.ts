import { atom } from "jotai";

// This is your shared state "cell"
export const textAtom = atom<string>("");

// This is a "derived" atom - it updates automatically whenever textAtom changes
export const charCountAtom = atom((get) => get(textAtom).length);
