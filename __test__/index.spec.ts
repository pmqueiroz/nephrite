import { describe, expect, it } from 'vitest';

import { type Config, Nephrite } from '../index';

const defaultConfig = {
  source: ['src/tokens/**/*.json'],
} satisfies Config;

describe('Nephrite', () => {
  describe('getConfig', () => {
    it('should be a function', () => {
      expect(Nephrite.prototype.getConfig).toBeInstanceOf(Function);
    });

    it('should return config on getConfig', () => {
      const nephrite = new Nephrite(defaultConfig);

      expect(nephrite.getConfig()).toEqual(
        expect.objectContaining(defaultConfig),
      );
    });
  });
});
