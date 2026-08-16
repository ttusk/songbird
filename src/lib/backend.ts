import { invoke } from "@tauri-apps/api/core";

export type BackendCampaign = {
  id: number;
  name: string;
  notes: string;
};

export type NewBackendCampaign = {
  name: string;
  notes: string | null;
};

export type BackendCampaignDetails = {
  campaign: BackendCampaign;
  characters: unknown[];
};

export function isTauriRuntime() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function listCampaigns(): Promise<BackendCampaign[] | null> {
  if (!isTauriRuntime()) {
    return null;
  }

  return invoke<BackendCampaign[]>("list_campaigns");
}

export async function addCampaign(
  campaign: NewBackendCampaign,
): Promise<BackendCampaign | null> {
  if (!isTauriRuntime()) {
    return null;
  }

  return invoke<BackendCampaign>("add_campaign", { campaign });
}

export async function findCampaignDetails(
  id: number,
): Promise<BackendCampaignDetails | null> {
  if (!isTauriRuntime()) {
    return null;
  }

  return invoke<BackendCampaignDetails | null>("find_campaign_details", { id });
}
