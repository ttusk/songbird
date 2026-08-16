import { useEffect, useState } from "react";
import { Home, Settings } from "lucide-react";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
} from "@/components/ui/sidebar";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import SettingsScreen, { type Theme } from "@/Settings";
import { translations, type Language } from "@/i18n";
import HomeScreen from "@/Home";
import "./App.css";

function App() {
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [theme, setTheme] = useState<Theme>(() =>
    localStorage.getItem("songbird.theme") === "dark" ? "dark" : "light",
  );
  const [language, setLanguage] = useState<Language>(() =>
    localStorage.getItem("songbird.language") === "pt" ? "pt" : "en",
  );
  const [volume, setVolume] = useState(
    () => localStorage.getItem("songbird.volume") ?? "75",
  );

  useEffect(() => {
    document.documentElement.classList.toggle("dark", theme === "dark");
    localStorage.setItem("songbird.theme", theme);
  }, [theme]);

  useEffect(() => {
    localStorage.setItem("songbird.volume", volume);
  }, [volume]);
  useEffect(() => {
    localStorage.setItem("songbird.language", language);
  }, [language]);
  const t = translations[language];
  return (
    <SidebarProvider open={false} onOpenChange={() => {}}>
      <Sidebar collapsible="icon">
        <SidebarContent>
          <SidebarGroup className="p-2">
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    tooltip={t.navigation.home}
                    isActive
                    onClick={() => setIsSettingsOpen(false)}
                  >
                    <Home />
                    <span>{t.navigation.home}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
        <SidebarFooter className="border-t p-2">
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton
                tooltip={t.navigation.settings}
                onClick={() => setIsSettingsOpen(true)}
              >
                <Settings />
                <span>{t.navigation.settings}</span>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarFooter>
      </Sidebar>

      <SidebarInset>
        <header className="flex h-14 items-center border-b px-4">
          <span className="text-sm font-medium">{t.navigation.home}</span>
        </header>
        <HomeScreen language={language} />
      </SidebarInset>
      <Dialog open={isSettingsOpen} onOpenChange={setIsSettingsOpen}>
        <DialogContent
          className="max-h-[90vh] overflow-y-auto sm:max-w-2xl"
          closeLabel={t.settings.close}
        >
          <DialogHeader>
            <DialogTitle>{t.settings.title}</DialogTitle>
            <DialogDescription>{t.settings.description}</DialogDescription>
          </DialogHeader>
          <SettingsScreen
            theme={theme}
            language={language}
            volume={volume}
            onThemeChange={setTheme}
            onLanguageChange={setLanguage}
            onVolumeChange={setVolume}
          />
        </DialogContent>
      </Dialog>
    </SidebarProvider>
  );
}

export default App;
