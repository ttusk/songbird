import { useEffect, useState } from "react";
import ReactMarkdown from "react-markdown";
import { ArrowLeft, FolderOpen } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  Card,
  CardContent,
} from "@/components/ui/card";
import { translations, type Language } from "@/i18n";
import {
  addCampaign,
  findCampaignDetails,
  isTauriRuntime,
  listCampaigns,
  type BackendCampaign,
} from "@/lib/backend";

export type Campaign = BackendCampaign;

export const mockCampaigns: Campaign[] = [
  {
    id: 1,
    name: "The Ember Crown",
    notes: "Recover the **map** before the tide turns.\n\nThe archive opens at low tide.",
  },
  {
    id: 2,
    name: "Ashes of Valewatch",
    notes: "A frontier town is keeping secrets beneath its chapel.\n\n*Ask the innkeeper about the bells.*",
  },
  {
    id: 3,
    name: "The Glass Sea",
    notes: "Follow the star chart to an island that appears once a decade.",
  },
];


function MarkdownContent({
  content,
  emptyText,
}: {
  content: string;
  emptyText: string;
}) {
  return (
    <div className="text-sm leading-6 text-muted-foreground [&_a]:text-primary [&_a]:underline [&_em]:italic [&_p:not(:last-child)]:mb-2 [&_strong]:font-semibold">
      <ReactMarkdown>{content || emptyText}</ReactMarkdown>
    </div>
  );
}
function CampaignDetailScreen({
  campaign,
  language,
  onBack,
}: {
  campaign: Campaign;
  language: Language;
  onBack: () => void;
}) {
  const t = translations[language].campaigns;
  return (
    <section className="mx-auto max-w-3xl">
      <Button type="button" variant="ghost" onClick={onBack}>
        <ArrowLeft data-icon="inline-start" />
        {t.back}
      </Button>
      <div className="mt-6">
        <h1 className="text-2xl font-semibold tracking-tight">{campaign.name}</h1>
        <div className="mt-6 rounded-lg border bg-muted/30 p-5 sm:p-6">
          <MarkdownContent content={campaign.notes} emptyText={t.noNotes} />
        </div>
      </div>
    </section>
  );
}



function HomeScreen({ language }: { language: Language }) {
  const [campaigns, setCampaigns] = useState<Campaign[]>(
    isTauriRuntime() ? [] : mockCampaigns,
  );
  const [isLoading, setIsLoading] = useState(isTauriRuntime());
  const [error, setError] = useState<string | null>(null);
  const t = translations[language].campaigns;
  const [isCreating, setIsCreating] = useState(false);
  const [isPreviewing, setIsPreviewing] = useState(false);
  const [selectedCampaign, setSelectedCampaign] = useState<Campaign | null>(null);
  const [name, setName] = useState("");
  const [notes, setNotes] = useState("");

  useEffect(() => {
    let active = true;

    listCampaigns()
      .then((loadedCampaigns) => {
        if (!active) {
          return;
        }

        if (loadedCampaigns) {
          setCampaigns(loadedCampaigns);
        }
        setIsLoading(false);
      })
      .catch(() => {
        if (active) {
          setError(t.loadError);
          setIsLoading(false);
        }
      });

    return () => {
      active = false;
    };
  }, [t.loadError]);

  const resetForm = () => {
    setName("");
    setNotes("");
    setIsPreviewing(false);
  };

  const createCampaign = async () => {
    const trimmedName = name.trim();

    if (!trimmedName) {
      return;
    }

    setError(null);

    try {
      const createdCampaign = await addCampaign({
        name: trimmedName,
        notes: notes.trim() || null,
      });
      const fallbackCampaign: Campaign = {
        id: Math.max(...campaigns.map((campaign) => campaign.id), 0) + 1,
        name: trimmedName,
        notes: notes.trim(),
      };

      setCampaigns((currentCampaigns) => [
        ...currentCampaigns,
        createdCampaign ?? fallbackCampaign,
      ]);
      resetForm();
      setIsCreating(false);
    } catch {
      setError(t.createError);
    }
  };

  const openCampaign = async (campaign: Campaign) => {
    setSelectedCampaign(campaign);

    if (!isTauriRuntime()) {
      return;
    }

    try {
      const details = await findCampaignDetails(campaign.id);
      if (details) {
        setSelectedCampaign(details.campaign);
      }
    } catch {
      setError(t.detailError);
    }
  };


  return (
    <div className="flex-1 overflow-auto bg-background text-foreground">
      <main className="mx-auto max-w-4xl p-6 sm:p-8" role="main">
        {selectedCampaign ? (
          <CampaignDetailScreen
            campaign={selectedCampaign}
            language={language}
            onBack={() => setSelectedCampaign(null)}
          />
        ) : (
          <>
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">{t.title}</h1>
        </div>


        <div className="mt-10 flex items-center justify-between">
          <h2 className="text-sm font-medium">{t.library}</h2>
          <div className="flex items-center gap-3">
          <Dialog
            open={isCreating}
            onOpenChange={(open) => {
              setIsCreating(open);
              if (!open) {
                resetForm();
              }
            }}
          >
            <DialogTrigger
              render={
                <Button type="button">
                  {t.newCampaign}
                </Button>
              }
            />
            <DialogContent className="sm:max-w-lg" closeLabel={t.close}>
              <DialogHeader>
                <DialogTitle>{t.newCampaign}</DialogTitle>
                <DialogDescription>{t.dialogDescription}</DialogDescription>
              </DialogHeader>
              <div className="grid gap-4 py-2">
                <Input
                  value={name}
                  placeholder={t.campaignName}
                  aria-label={t.campaignName}
                  onChange={(event) => setName(event.target.value)}
                />
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-sm font-medium">{t.notes}</span>
                    <div className="flex gap-1">
                      <Button
                        type="button"
                        size="sm"
                        variant={isPreviewing ? "ghost" : "secondary"}
                        onClick={() => setIsPreviewing(false)}
                      >
                        {t.write}
                      </Button>
                      <Button
                        type="button"
                        size="sm"
                        variant={isPreviewing ? "secondary" : "ghost"}
                        onClick={() => setIsPreviewing(true)}
                      >
                        {t.preview}
                      </Button>
                    </div>
                  </div>
                  {isPreviewing ? (
                    <div className="min-h-32 rounded-md border bg-muted/30 p-3">
                      <MarkdownContent content={notes} emptyText={t.noNotes} />
                    </div>
                  ) : (
                    <Textarea
                      value={notes}
                      placeholder={t.notesPlaceholder}
                      aria-label={t.notes}
                      className="min-h-32 resize-y"
                      onChange={(event) => setNotes(event.target.value)}
                    />
                  )}
                </div>
              </div>
              <DialogFooter>
                <Button type="button" variant="outline" onClick={() => setIsCreating(false)}>
                  {t.cancel}
                </Button>
                <Button type="button" disabled={!name.trim()} onClick={createCampaign}>
                  {t.create}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
          </div>
        </div>
        {error && (
          <p className="mt-4 text-sm text-destructive" role="alert">
            {error}
          </p>
        )}
        {isLoading ? (
          <p className="mt-8 text-sm text-muted-foreground">{t.loading}</p>
        ) : campaigns.length === 0 ? (
          <p className="mt-8 text-sm text-muted-foreground">{t.empty}</p>
        ) : (
          <div className="mt-3 grid gap-3 sm:grid-cols-2">
            {campaigns.map((campaign) => (
              <Card key={campaign.id}>
                <CardContent className="flex min-h-20 items-center justify-between gap-3 p-4">
                  <p className="min-w-0 truncate text-sm font-medium">{campaign.name}</p>
                  <Tooltip>
                    <TooltipTrigger
                      render={
                        <Button
                          type="button"
                          variant="outline"
                          size="icon-sm"
                          aria-label={`${t.open} ${campaign.name}`}
                          onClick={() => void openCampaign(campaign)}
                        />
                      }
                    >
                      <FolderOpen />
                    </TooltipTrigger>
                    <TooltipContent>{t.openCampaign}</TooltipContent>
                  </Tooltip>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
          </>
        )}
      </main>
    </div>
  );
}

export default HomeScreen;
