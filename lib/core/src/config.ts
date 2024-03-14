/*
 * Copyright (c) 2023 OneLiteFeatherNET
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

import { FeedbackFusionClient } from "./";
import en from "./locales/en";

interface BaseConfig {
  baseURL: string;
  target: string;
}

interface LocaleData {
  locale: string;
  translation: Object[];
}

interface ThemeOptions {
  [key: string]: {
    text: string;
    subtitle: string;
    sheet: string;
    primary: string;
    inactive: string;
    success: string;
    error: string;
  };
}

export interface FeedbackFusionConfigurationOptions extends BaseConfig {
  locales?: LocaleData[];
  defaultLocale?: string;
  defaultTheme?: string;
  themes?: ThemeOptions;
}

export interface FeedbackFusionConfig extends BaseConfig {
  locales: { [key: string]: { translation: Object } };
  defaultLocale: string;
  defaultTheme: string;
  themes: ThemeOptions;
}

const defaultThemes = {
  dark: {
    text: "#FFFFF5",
    subtitle: "#757575",
    sheet: "#212121",
    primary: "#3498db",
    inactive: "#757575",
    success: "#4caf50",
    error: "#d33d3d",
  },
};

export function patchConfig(
  config: FeedbackFusionConfigurationOptions,
): FeedbackFusionConfig {
  // default themes
  config.themes = Object.assign(defaultThemes, config.themes || {});
  config.defaultTheme = config.defaultTheme || "dark";

  // default locales
  if (!config.defaultLocale) {
    config.defaultLocale = "en";

    if (!config.locales?.find((locale: LocaleData) => locale.locale === "en")) {
      if (!config.locales) config.locales = [];
      // @ts-ignore
      config.locales.push(en);
    }
  }

  // transform the locales
  const locales: any = {};
  config.locales!.forEach((locale: LocaleData) =>
    locales[locale.locale] = { translation: locale.translation }
  );
  config.locales = locales;

  // @ts-ignore
  return config as FeedbackFusionConfig;
}

export interface FeedbackFusionState {
  config: FeedbackFusionConfig;
  client: FeedbackFusionClient;
}
