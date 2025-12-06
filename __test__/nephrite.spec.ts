import fs from 'node:fs/promises';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { type Config, Nephrite } from '../index';
import { setupTokensDir } from './utils/setup-tokens-dir'

let tempDir: string;

const defaultConfig = () =>
  ({
    source: ['src/tokens/**/*.json'],
    cwd: tempDir,
  }) satisfies Config;

describe('Nephrite', () => {
  beforeEach(async () => {
    tempDir = await setupTokensDir();
  });

  afterEach(async () => {
    await fs.rm(tempDir, { recursive: true, force: true });
  });

  describe('getConfig', () => {
    it('should be a function', () => {
      expect(Nephrite.prototype.getConfig).toBeInstanceOf(Function);
    });

    it('should return config on getConfig', () => {
      const nephrite = new Nephrite(defaultConfig());

      expect(nephrite.getConfig()).toEqual(
        expect.objectContaining(defaultConfig()),
      );
    });
  });

  describe('build', () => {
    it('should be a function', () => {
      expect(Nephrite.prototype.build).toBeInstanceOf(Function);
    });

    it('should [wip]', async () => {
      const nephrite = new Nephrite(defaultConfig());
      nephrite.build();
    });
  });
});
