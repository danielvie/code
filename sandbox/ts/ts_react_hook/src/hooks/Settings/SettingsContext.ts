import { createContext } from "react";

export type SettingsContextType = {
  theme: "light" | "dark";
  language: "en" | "es";
  toggleTheme: () => void;
  setLanguage: (lang: "en" | "es") => void;
};

export const SettingsContext = createContext<SettingsContextType | null>(null);
