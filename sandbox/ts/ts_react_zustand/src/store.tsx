import { create } from "zustand";
import type { TTab } from "./types";

interface AppState {
  activeTab: TTab;
  setTab: (newTab: TTab) => void;
}

const useStore = create<AppState>((set) => ({
  // initial state
  activeTab: "Home",

  // action to update the state
  setTab: (newTab) => set({ activeTab: newTab }),
}));

export default useStore;
