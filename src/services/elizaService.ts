// Custom implementation of Eliza chatbot for AgentX yield agent
// This is a sophisticated version that focuses on yield-related conversations

class ElizaService {
  private history: { type: 'user' | 'agent', message: string }[] = [];
  private context: Record<string, any> = {};
  
  // More comprehensive keyword patterns with detailed responses
  private keywords: Record<string, string[]> = {
    yield: [
      "Currently, the highest APY is offered by JEXchange at 28.5% for JEX-USDC Farm, though it carries higher risk.",
      "For lower risk options, consider Hatom Protocol's EGLD Lending at 5.8% APY.",
      "The average APY across all MultiversX protocols is currently around 14.2%.",
      "Would you prefer stable yields or are you comfortable with more volatile returns?",
      "I can help you find the optimal yield strategy based on your risk tolerance. What's your preferred risk level?"
    ],
    apy: [
      "Currently, the highest APY is offered by JEXchange at 28.5% for JEX-USDC Farm, though it carries higher risk.",
      "For lower risk options, consider Hatom Protocol's EGLD Lending at 5.8% APY.",
      "The average APY across all MultiversX protocols is currently around 14.2%.",
      "APY rates vary by protocol. Maiar Exchange offers around 12.5% APY with medium risk, while xExchange offers 14.3% with similar risk.",
      "Would you like me to compare APY rates across different protocols for a specific token?"
    ],
    risk: [
      "I can adjust your portfolio to focus on low-risk opportunities like stablecoin pools.",
      "Medium risk options like EGLD-MEX LP offer a balance of security and returns.",
      "High risk pools can offer APYs above 20% but with greater volatility and potential impermanent loss.",
      "What's your risk tolerance on a scale from conservative to aggressive?",
      "For a conservative strategy, I recommend allocating at least 60% to low-risk options like Hatom's lending pools."
    ],
    rebalance: [
      "I can automatically rebalance your portfolio daily, weekly, or monthly.",
      "When would you like me to rebalance your positions to maximize returns?",
      "Frequent rebalancing can capture more yield but may incur higher transaction costs.",
      "I'll monitor market conditions and rebalance when significant opportunities arise.",
      "The optimal rebalancing frequency depends on market volatility and your investment size. For most users, weekly is a good balance."
    ],
    strategy: [
      "I recommend diversifying across at least 3-4 different protocols for optimal risk management.",
      "A barbell strategy with both stable assets and higher-yield opportunities might work well for you.",
      "Would you like me to focus on a particular token or spread investments across the ecosystem?",
      "I can create a custom strategy based on your time horizon and financial goals.",
      "For long-term growth, I suggest a balanced approach with 40% in lending, 40% in liquidity pools, and 20% in higher-risk yield farming."
    ],
    protocol: [
      "The MultiversX ecosystem has several established protocols like Maiar Exchange, Hatom, and xExchange.",
      "Each protocol has different risk profiles and yield opportunities.",
      "Would you like me to recommend specific protocols based on your investment goals?",
      "I can help you distribute your assets across multiple protocols for diversification.",
      "Hatom Protocol is excellent for lending, while Maiar Exchange offers strong liquidity pools. Which aspect interests you more?"
    ],
    invest: [
      "I can help you invest in the highest yielding opportunities while managing risk.",
      "What amount are you looking to invest in MultiversX DeFi?",
      "For new investors, I recommend starting with lower-risk protocols like Hatom's lending pools.",
      "Would you like me to create a diversified investment strategy across multiple protocols?",
      "With the current market conditions, a mix of stablecoin lending and EGLD liquidity pools offers a good balance of safety and returns."
    ],
    egld: [
      "EGLD is the native token of the MultiversX blockchain with a current price of $42.75.",
      "There are several EGLD yield opportunities, including Hatom lending at 5.8% APY and EGLD-MEX LP at 18.5% APY.",
      "EGLD staking is also available with validator nodes offering around 8-10% APY.",
      "Would you like me to recommend specific EGLD yield strategies?",
      "For EGLD holders, I recommend diversifying between lending, liquidity provision, and staking to optimize returns while managing risk."
    ],
    stablecoin: [
      "Stablecoin pools on AshSwap offer around 9.2% APY with low risk.",
      "USDC lending on Hatom Protocol provides 8.7% APY.",
      "Stablecoins provide lower but more consistent returns compared to volatile assets.",
      "Would you like me to allocate a portion of your portfolio to stablecoin strategies?",
      "Stablecoins are excellent for reducing portfolio volatility while still generating meaningful yield."
    ],
    hatom: [
      "Hatom Protocol is a lending and borrowing platform on MultiversX.",
      "They offer competitive rates for EGLD (5.8% APY) and USDC (8.7% APY).",
      "Hatom has a medium risk profile with $78.3M in total value locked.",
      "Would you like to include Hatom in your yield strategy?",
      "Hatom Protocol is one of the most established lending platforms on MultiversX with a strong security track record."
    ],
    maiar: [
      "Maiar Exchange (now xExchange) is the leading DEX on MultiversX.",
      "Their EGLD-MEX liquidity pool offers 18.5% APY with medium risk.",
      "They have $124.5M in total value locked across all pools.",
      "Would you like to explore Maiar Exchange liquidity pools?",
      "Maiar Exchange offers some of the most liquid trading pairs on MultiversX, which helps reduce impermanent loss risk."
    ],
    compare: [
      "When comparing protocols, I look at APY, risk level, TVL, and historical performance.",
      "Would you like me to compare specific protocols or investment strategies?",
      "Hatom offers lower but more stable returns compared to xExchange, which has higher APY but greater volatility.",
      "For a fair comparison, we should consider both returns and risks. Which aspect is more important to you?",
      "I can provide a detailed comparison of any protocols you're interested in. Which ones would you like to compare?"
    ],
    portfolio: [
      "I can help you optimize your portfolio based on your risk tolerance and investment goals.",
      "A well-balanced portfolio typically includes a mix of lending, liquidity provision, and some yield farming.",
      "Would you like me to analyze your current positions and suggest improvements?",
      "For long-term growth, I recommend diversifying across at least 3-4 different protocols.",
      "What's your current portfolio allocation, and what are you looking to improve?"
    ],
    impermanent: [
      "Impermanent loss occurs when the price of your tokens changes compared to when you deposited them in a liquidity pool.",
      "To minimize impermanent loss, consider stable pairs or protocols with IL protection mechanisms.",
      "Would you like me to recommend pools with lower impermanent loss risk?",
      "Stablecoin pairs have minimal impermanent loss risk, making them ideal for risk-averse investors.",
      "Impermanent loss can be offset by trading fees and incentive rewards in high-volume pools."
    ],
    fees: [
      "Transaction fees on MultiversX are typically very low, around 0.0005 EGLD per transaction.",
      "Some protocols charge performance fees on yields, usually between 5-20%.",
      "Would you like me to factor in fees when recommending yield strategies?",
      "When comparing APY across protocols, I always consider the net return after all fees.",
      "Gas fees for rebalancing are minimal on MultiversX, making frequent optimization viable."
    ],
    hello: [
      "Hello! I'm your AgentX yield optimization assistant. How can I help you today?",
      "Hi there! I'm ready to help you maximize your MultiversX yields. What would you like to know?",
      "Greetings! I'm your personal DeFi assistant. What aspect of yield farming are you interested in?",
      "Hello! I'm here to help you navigate the MultiversX DeFi ecosystem. What can I assist you with?",
      "Hi! I'm your AgentX assistant. Would you like me to recommend some yield opportunities or explain how our platform works?"
    ],
    help: [
      "I can help with many things! I can recommend yield strategies, compare protocols, explain DeFi concepts, or analyze your portfolio.",
      "Need assistance? I can help you find the best yields, understand risks, or create a custom investment strategy.",
      "I'm here to help! You can ask me about specific protocols, APY rates, risk management, or portfolio optimization.",
      "How can I assist you today? I can provide information on MultiversX DeFi, recommend investment strategies, or explain yield farming concepts.",
      "I can help with yield optimization, risk assessment, protocol comparisons, and much more. What specific area are you interested in?"
    ],
    thanks: [
      "You're welcome! Is there anything else I can help you with?",
      "Happy to help! Let me know if you have any other questions.",
      "My pleasure! Is there anything else you'd like to know about MultiversX yield opportunities?",
      "Glad I could assist! Feel free to ask if you need more information.",
      "You're welcome! I'm here whenever you need DeFi guidance or yield recommendations."
    ]
  };

  // More sophisticated fallback responses that probe for user intent
  private fallbackResponses: string[] = [
    "I'm analyzing the best yield opportunities for you. Can you tell me more about your investment goals?",
    "That's interesting. Would you prefer higher yields with more risk, or stable returns with lower risk?",
    "I can help optimize your MultiversX portfolio. What's your time horizon for these investments?",
    "Several protocols on MultiversX offer competitive yields. Are you interested in any specific tokens or protocols?",
    "I'm here to help you navigate the MultiversX DeFi ecosystem. What aspects are you most interested in learning about?",
    "Based on current market conditions, I'd recommend diversifying across 3-4 protocols. Would you like specific suggestions?",
    "Your yield agent can automatically rebalance your portfolio. How frequently would you prefer this to happen?",
    "I'm continuously monitoring MultiversX protocols for the best opportunities. What's your primary investment goal?",
    "To provide better recommendations, could you share your risk tolerance level? Are you conservative, moderate, or aggressive?",
    "I'd like to understand your needs better. Are you looking for long-term growth, stable income, or maximum short-term yields?",
    "Different protocols excel at different strategies. Are you more interested in lending, liquidity provision, or yield farming?",
    "I can provide more personalized advice if you share your investment timeframe. Are you looking at weeks, months, or years?",
    "The MultiversX ecosystem offers various yield opportunities. Which aspect would you like me to explain in more detail?"
  ];

  // Conversation flow patterns to make responses more contextual
  private conversationPatterns: Record<string, (message: string) => string | null> = {
    askRiskTolerance: (message: string) => {
      const riskTerms = ['risk', 'conservative', 'aggressive', 'moderate', 'safe', 'risky'];
      if (riskTerms.some(term => message.toLowerCase().includes(term))) {
        if (message.toLowerCase().includes('conservative') || message.toLowerCase().includes('safe') || message.toLowerCase().includes('low risk')) {
          this.context.riskTolerance = 'conservative';
          return "I understand you prefer a conservative approach. I'll focus on lower-risk opportunities like Hatom's lending pools and stablecoin strategies, which typically offer 5-9% APY with minimal volatility. Would you like specific recommendations for conservative strategies?";
        } else if (message.toLowerCase().includes('aggressive') || message.toLowerCase().includes('risky') || message.toLowerCase().includes('high risk')) {
          this.context.riskTolerance = 'aggressive';
          return "I see you're comfortable with an aggressive strategy. I can recommend higher-yield opportunities like JEXchange's JEX-USDC farm (28.5% APY) and xExchange's EGLD-XEX farm (22.4% APY). These carry higher risk but potentially greater rewards. Would you like more details on these opportunities?";
        } else if (message.toLowerCase().includes('moderate') || message.toLowerCase().includes('balanced') || message.toLowerCase().includes('medium risk')) {
          this.context.riskTolerance = 'balanced';
          return "A balanced approach is a great choice. I recommend a mix of lending protocols like Hatom (8.7% APY for USDC) and established liquidity pools like EGLD-MEX on Maiar Exchange (18.5% APY). This gives you a good balance of stability and growth. Would you like me to create a sample portfolio with this approach?";
        }
      }
      return null;
    },
    
    askInvestmentAmount: (message: string) => {
      // Look for mentions of investment amounts
      const amountMatch = message.match(/(\d+(\.\d+)?)\s*(egld|dollars|usd|\$)/i);
      if (amountMatch) {
        const amount = parseFloat(amountMatch[1]);
        const currency = amountMatch[3].toLowerCase();
        this.context.investmentAmount = amount;
        this.context.investmentCurrency = currency === 'egld' ? 'EGLD' : 'USD';
        
        if (amount < 10) {
          return `I see you're looking to invest ${amount} ${this.context.investmentCurrency}. For smaller amounts, I recommend focusing on a single protocol to minimize transaction costs. Hatom's EGLD lending at 5.8% APY would be a good starting point. Would you like more options?`;
        } else if (amount < 100) {
          return `With ${amount} ${this.context.investmentCurrency}, you can start diversifying across 2-3 protocols. I'd suggest allocating 50% to Hatom lending and 50% to a stable liquidity pool like EGLD-USDC on Maiar Exchange. Would this approach work for you?`;
        } else {
          return `With ${amount} ${this.context.investmentCurrency}, you can build a well-diversified portfolio. I recommend 40% in lending protocols, 40% in established liquidity pools, and 20% in higher-yield opportunities. Would you like me to create a detailed allocation plan?`;
        }
      }
      return null;
    },
    
    askTimeHorizon: (message: string) => {
      const shortTermPatterns = ['short term', 'short-term', 'quick', 'few days', 'few weeks', 'short period'];
      const mediumTermPatterns = ['medium term', 'medium-term', 'few months', 'half year', 'months'];
      const longTermPatterns = ['long term', 'long-term', 'year', 'years', 'long period'];
      
      if (shortTermPatterns.some(term => message.toLowerCase().includes(term))) {
        this.context.timeHorizon = 'short';
        return "For short-term investments, liquidity is key. I recommend stablecoin pools on AshSwap (9.2% APY) or USDC lending on Hatom (8.7% APY). These allow you to exit positions quickly with minimal price risk. Would you like more short-term options?";
      } else if (mediumTermPatterns.some(term => message.toLowerCase().includes(term))) {
        this.context.timeHorizon = 'medium';
        return "For a medium-term horizon of a few months, balanced liquidity pools like EGLD-MEX on Maiar Exchange (18.5% APY) offer a good compromise between yield and stability. Would you like me to suggest a medium-term portfolio allocation?";
      } else if (longTermPatterns.some(term => message.toLowerCase().includes(term))) {
        this.context.timeHorizon = 'long';
        return "With a long-term investment horizon, you can benefit from compounding and ride out short-term volatility. I recommend a diversified approach with 30% in lending, 40% in liquidity pools, and 30% in higher-yield farming opportunities. This could yield 15-20% APY on average over time. Would you like a detailed long-term strategy?";
      }
      return null;
    },
    
    askSpecificProtocol: (message: string) => {
      const protocols = {
        'maiar': "Maiar Exchange is the leading DEX on MultiversX with $124.5M TVL. Their EGLD-MEX liquidity pool offers 18.5% APY with medium risk. They also offer farms with additional MEX rewards. Would you like specific pool recommendations from Maiar?",
        'hatom': "Hatom Protocol is a lending and borrowing platform with $78.3M TVL. They offer 5.8% APY for EGLD lending and 8.7% for USDC, both with relatively low risk. Their auto-compounding feature maximizes your returns. Would you like to know more about Hatom's lending options?",
        'ashswap': "AshSwap is a stable swap AMM focused on minimal slippage with $45.6M TVL. Their stablecoin pool offers 9.2% APY with low risk, making it ideal for conservative investors. Would you like more details about AshSwap's pools?",
        'xexchange': "xExchange is a decentralized exchange with $92.1M TVL offering farming opportunities and governance. Their EGLD-XEX farm offers 22.4% APY but with higher risk. Would you like to explore xExchange's yield options?",
        'onedex': "OneDex is an aggregator DEX providing optimal rates across MultiversX with $31.5M TVL. Their EGLD-ONE LP offers 14.8% APY with medium risk. Would you like more information about OneDex?",
        'jexchange': "JEXchange focuses on community governance and yield farming with $28.7M TVL. Their JEX-USDC farm offers the highest APY in the ecosystem at 28.5%, but with higher risk. Would you like to know more about JEXchange's high-yield opportunities?"
      };
      
      for (const [protocol, response] of Object.entries(protocols)) {
        if (message.toLowerCase().includes(protocol)) {
          this.context.lastProtocolDiscussed = protocol;
          return response;
        }
      }
      return null;
    },
    
    providePortfolioRecommendation: (message: string) => {
      if (message.toLowerCase().includes('portfolio') || 
          message.toLowerCase().includes('recommend') || 
          message.toLowerCase().includes('suggest') || 
          message.toLowerCase().includes('allocation')) {
        
        // Use context if available
        const riskTolerance = this.context.riskTolerance || 'balanced';
        const timeHorizon = this.context.timeHorizon || 'medium';
        
        if (riskTolerance === 'conservative') {
          return "Based on your conservative risk profile, I recommend this portfolio allocation:\n\n" +
                 "• 50% - Hatom USDC lending (8.7% APY)\n" +
                 "• 30% - AshSwap stablecoin pool (9.2% APY)\n" +
                 "• 20% - Maiar EGLD-USDC LP (12.3% APY)\n\n" +
                 "This gives you a weighted average APY of about 9.5% with minimal risk. Would you like me to explain any of these options in more detail?";
        } else if (riskTolerance === 'aggressive') {
          return "For your aggressive risk profile, I recommend this high-yield portfolio:\n\n" +
                 "• 40% - xExchange EGLD-XEX farm (22.4% APY)\n" +
                 "• 30% - JEXchange JEX-USDC farm (28.5% APY)\n" +
                 "• 20% - Maiar EGLD-MEX LP (18.5% APY)\n" +
                 "• 10% - Hatom EGLD lending (5.8% APY) as a safety buffer\n\n" +
                 "This gives you a weighted average APY of about 21.7%. Would you like me to adjust this allocation?";
        } else {
          return "For a balanced approach, I recommend this diversified portfolio:\n\n" +
                 "• 30% - Hatom USDC lending (8.7% APY)\n" +
                 "• 30% - Maiar EGLD-MEX LP (18.5% APY)\n" +
                 "• 20% - OneDex EGLD-ONE LP (14.8% APY)\n" +
                 "• 20% - xExchange EGLD-XEX farm (22.4% APY)\n\n" +
                 "This gives you a weighted average APY of about 15.6% with moderate risk. Would you like me to explain the rationale behind this allocation?";
        }
      }
      return null;
    }
  };

  constructor() {
    // Add initial greeting to history
    const initialGreeting = "Hello! I'm your AgentX yield optimization assistant. I can help you maximize returns while managing risk. What would you like to know about yield opportunities?";
    this.history.push({ type: 'agent', message: initialGreeting });
  }

  sendMessage(message: string): string {
    // Add user message to history
    this.history.push({ type: 'user', message });
    
    // Generate response
    const response = this.generateResponse(message);
    
    // Add agent response to history
    this.history.push({ type: 'agent', message: response });
    
    return response;
  }

  private generateResponse(message: string): string {
    const lowerMessage = message.toLowerCase();
    
    // First, check for conversation patterns to provide contextual responses
    for (const patternFn of Object.values(this.conversationPatterns)) {
      const response = patternFn(message);
      if (response) {
        return response;
      }
    }
    
    // Check for keyword matches
    for (const [keyword, responses] of Object.entries(this.keywords)) {
      if (lowerMessage.includes(keyword)) {
        // Return a response for the matched keyword
        return responses[Math.floor(Math.random() * responses.length)];
      }
    }
    
    // If no keywords matched, use a fallback response
    return this.fallbackResponses[Math.floor(Math.random() * this.fallbackResponses.length)];
  }

  getHistory(): { type: 'user' | 'agent', message: string }[] {
    return this.history;
  }

  reset(): void {
    this.history = [];
    this.context = {};
    
    // Add initial greeting to history
    const initialGreeting = "Hello! I'm your AgentX yield optimization assistant. I can help you maximize returns while managing risk. What would you like to know about yield opportunities?";
    this.history.push({ type: 'agent', message: initialGreeting });
  }
}

// Create a singleton instance
const elizaService = new ElizaService();

export default elizaService;