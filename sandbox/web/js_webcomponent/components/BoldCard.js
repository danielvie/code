class BoldCard extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  static get observedAttributes() {
    return ['title', 'value', 'status', 'icon', 'color'];
  }

  attributeChangedCallback() {
    this.render();
  }

  connectedCallback() {
    this.render();
  }

  render() {
    const title = this.getAttribute('title') || 'METRIC';
    const value = this.getAttribute('value') || '0';
    const status = this.getAttribute('status') || 'normal';
    const icon = this.getAttribute('icon') || '⚡';
    const cardColor = this.getAttribute('color') || '#fbbf24';

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }

        .card {
          background: white;
          border: 4px solid black;
          box-shadow: 8px 8px 0px black;
          padding: 24px;
          transition: all 0.1s ease;
          display: flex;
          flex-direction: column;
          gap: 16px;
          height: 100%;
          position: relative;
          user-select: none;
          -webkit-user-select: none;
        }

        .card:hover {
          transform: translate(-4px, -4px);
          box-shadow: 12px 12px 0px black;
        }

        .card:active {
          transform: translate(2px, 2px);
          box-shadow: 4px 4px 0px black;
        }

        .header {
          display: flex;
          justify-content: space-between;
          align-items: flex-start;
        }

        .icon-box {
          width: 64px;
          height: 64px;
          background: ${cardColor};
          border: 4px solid black;
          display: flex;
          align-items: center;
          justify-content: center;
          font-size: 32px;
          box-shadow: 4px 4px 0px black;
        }

        .status-badge {
          background: black;
          color: white;
          padding: 4px 12px;
          font-weight: 900;
          font-size: 12px;
          text-transform: uppercase;
          transform: rotate(2deg);
        }

        .content {
          margin-top: 8px;
        }

        .title {
          font-weight: 700;
          font-size: 16px;
          text-transform: uppercase;
          letter-spacing: -0.5px;
          color: #666;
          margin-bottom: 4px;
        }

        .value {
          font-weight: 900;
          font-size: 48px;
          line-height: 1;
          word-break: break-all;
        }

        .footer {
          margin-top: auto;
          border-top: 4px solid black;
          padding-top: 16px;
          display: flex;
          align-items: center;
          gap: 8px;
          font-weight: 700;
          font-size: 14px;
        }

        .dot {
          width: 12px;
          height: 12px;
          background: ${cardColor};
          border: 2px solid black;
        }
      </style>
      <div class="card">
        <div class="header">
          <div class="icon-box">${icon}</div>
          <div class="status-badge">${status}</div>
        </div>
        <div class="content">
          <div class="title">${title}</div>
          <div class="value">${value}</div>
        </div>
        <div class="footer">
          <div class="dot"></div>
          LIVE_TELEMETRY_STREAM
        </div>
      </div>
    `;
  }
}

customElements.define('bold-card', BoldCard);
