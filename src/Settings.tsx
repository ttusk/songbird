import type { ChangeEvent } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { translations, type Language } from "@/i18n";

export type Theme = "light" | "dark";

type SettingsScreenProps = {
  theme: Theme;
  language: Language;
  volume: string;
  onThemeChange: (theme: Theme) => void;
  onLanguageChange: (language: Language) => void;
  onVolumeChange: (volume: string) => void;
};

function SettingsScreen({
  theme,
  language,
  volume,
  onThemeChange,
  onLanguageChange,
  onVolumeChange,
}: SettingsScreenProps) {
  const t = translations[language].settings;
  const handleVolumeChange = (event: ChangeEvent<HTMLInputElement>) => {
    onVolumeChange(event.target.value);
  };

  return (
    <div className="grid gap-6">
      <section className="space-y-3">
        <div>
          <h2 className="text-sm font-medium">{t.appearance}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.appearanceDescription}</p>
        </div>
        <div className="flex gap-2">
          <Button
            type="button"
            variant={theme === "light" ? "secondary" : "outline"}
            onClick={() => onThemeChange("light")}
          >
            {t.light}
          </Button>
          <Button
            type="button"
            variant={theme === "dark" ? "secondary" : "outline"}
            onClick={() => onThemeChange("dark")}
          >
            {t.dark}
          </Button>
        </div>
      </section>
      <section className="space-y-3 border-t pt-6">
        <div>
          <h2 className="text-sm font-medium">{t.language}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.languageDescription}</p>
        </div>
        <div className="flex flex-wrap gap-2">
          <Button
            type="button"
            className="min-w-24"
            variant={language === "en" ? "secondary" : "outline"}
            onClick={() => onLanguageChange("en")}
          >
            {t.english}
          </Button>
          <Button
            type="button"
            className="min-w-24"
            variant={language === "pt" ? "secondary" : "outline"}
            onClick={() => onLanguageChange("pt")}
          >
            {t.portuguese}
          </Button>
        </div>
      </section>


      <section className="space-y-3 border-t pt-6">
        <div>
          <h2 className="text-sm font-medium">{t.audio}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.audioDescription}</p>
        </div>
        <div className="space-y-2">
          <div className="flex items-center justify-between text-sm">
            <span>{t.masterVolume}</span>
            <span className="text-muted-foreground">{volume}%</span>
          </div>
          <Input
            type="range"
            min="0"
            max="100"
            value={volume}
            aria-label={t.masterVolume}
            onChange={handleVolumeChange}
          />
        </div>
      </section>

      <section className="space-y-1 border-t pt-6">
        <h2 className="text-sm font-medium">{t.data}</h2>
        <p className="text-sm text-muted-foreground">{t.dataDescription}</p>
      </section>

      <footer className="border-t pt-4 text-center text-xs text-muted-foreground">
        {t.version}
      </footer>
    </div>
  );
}

export default SettingsScreen;
