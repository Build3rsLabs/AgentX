declare module 'elizabot' {
  export class ElizaBot {
    constructor(config?: {
      initial?: string;
      final?: string;
      keywords?: any[];
      synonyms?: Record<string, string[]>;
      postTransforms?: any[];
    });
    
    getInitial(): string;
    getFinal(): string;
    transform(text: string): string;
    reset(): void;
  }
}