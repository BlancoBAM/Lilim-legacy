/**
 * Lilim API Client
 * Backend communication for Lilim AI Assistant
 */

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export interface LilimQuery {
  text: string;
  session_id?: string;
  tools_enabled?: boolean;
  yolo_mode?: boolean;
}

export interface LilimResponse {
  response: string;
  source: string;
  domain: string;
}

export class LilimAPIError extends Error {
  constructor(message: string, public statusCode?: number) {
    super(message);
    this.name = 'LilimAPIError';
  }
}

/**
 * Send a chat query to Lilim backend
 */
export async function sendQuery(query: LilimQuery): Promise<LilimResponse> {
  try {
    const response = await fetch(`${API_BASE_URL}/chat`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(query),
    });

    if (!response.ok) {
      throw new LilimAPIError(
        `API request failed: ${response.statusText}`,
        response.status
      );
    }

    const data = await response.json();
    return data;
  } catch (error) {
    if (error instanceof LilimAPIError) {
      throw error;
    }
    
    if (error instanceof TypeError && error.message.includes('fetch')) {
      throw new LilimAPIError(
        'Cannot connect to Lilim server. Is the service running?'
      );
    }
    
    throw new LilimAPIError(
      `Unexpected error: ${error instanceof Error ? error.message : String(error)}`
    );
  }
}

/**
 * Check if Lilim backend is reachable
 */
export async function healthCheck(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE_URL}/health`, {
      method: 'GET',
    });
    return response.ok;
  } catch {
    return false;
  }
}

/**
 * Get or create a session ID
 */
export function getSessionId(): string {
  let sessionId = localStorage.getItem('lilim_session_id');
  
  if (!sessionId) {
    sessionId = `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    localStorage.setItem('lilim_session_id', sessionId);
  }
  
  return sessionId;
}

/**
 * Clear current session
 */
export function clearSession(): void {
  localStorage.removeItem('lilim_session_id');
}
