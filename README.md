# TrustLedger

**Financial Identity Infrastructure for Ghana's Informal Economy**

---

## Overview

TrustLedger is a financial operating system that digitizes susu groups, market trader savings, and rotational savings systems (ROSCAs) in Ghana. It replaces paper notebooks with digital record-keeping, builds trust scores that create financial identity for the unbanked, and enables automated payouts via Moolre's payment infrastructure.

**Live API Integration**: TrustLedger connects directly to Moolre's live API environment, enabling real money movement, SMS OTP verification, and production-ready payment flows.

---

## The Problem We're Solving

In Ghana today:

- **80% of the workforce** operates in the informal economy
- **Susu collectors** carry notebooks with thousands of cedis in cash
- **Notebooks get lost** — savings disappear overnight
- **No credit history** means no loans, no formal financial services
- **Momo payments exist** but no system ties them to group savings

### Real Example

> A tomato trader at Makola Market saves GHS 50 weekly with her susu group of 20 women. After 6 months, the collector loses the notebook. GHS 24,000 in savings — gone. No record. No recourse.

TrustLedger prevents this.

---

## What TrustLedger Does

### For Susu Group Members (via USSD)

| Action           | How it works                      |
| ---------------- | --------------------------------- |
| Join group       | Dial \*713# → enter group code    |
| Pay contribution | USSD menu → amount → Momo payment |
| Check balance    | Dial \*713# → "My Balance"        |
| View trust score | Dial \*713# → "My Trust Score"    |

No smartphone required. Works on any phone.

### For Group Admins (via Web Dashboard)

| Feature             | What they see                                        |
| ------------------- | ---------------------------------------------------- |
| Group overview      | Members, total saved, next payout date               |
| Member trust scores | Color-coded indicators with explanations             |
| Transaction history | Every contribution recorded permanently              |
| One-click payouts   | Automatic disbursement to all members via Moolre API |

### For Members (via Web Portal)

| Feature       | What they can do                                     |
| ------------- | ---------------------------------------------------- |
| Dashboard     | View savings, groups, trust score                    |
| Groups        | Join groups, view members, make contributions        |
| Savings Goals | Create personal savings goals with strict/loose mode |
| Profile       | Update name and profile picture                      |
| Transactions  | View contribution and payout history                 |
| Trust Score   | View score history and improvement reasons           |

---

## Key Innovation: Trust Scores

Every member gets a trust score calculated from:

```
(On-time payments / Expected payments) × 100
```

| Range     | Label        | Display            |
| --------- | ------------ | ------------------ |
| 90-100%   | High Trust   | White badge        |
| 70-89%    | Medium Trust | Light gray badge   |
| 50-69%    | Low Trust    | Dark gray badge    |
| Below 50% | Poor Trust   | Darkest gray badge |

This trust score follows the member across groups and becomes their first financial identity. Banks don't serve these people — but TrustLedger does.

---

## Features Built

### Backend (Rust + Loco)

| Feature                   | Status      | Description                                         |
| ------------------------- | ----------- | --------------------------------------------------- |
| Admin Authentication      | ✅ Complete | JWT-based login with bcrypt                         |
| Member OTP Authentication | ✅ Complete | Phone number + OTP via SMS (live Moolre SMS API)    |
| Group Management          | ✅ Complete | Create, list, view groups                           |
| Member Management         | ✅ Complete | Add members, list by group, trust scores            |
| Contribution Recording    | ✅ Complete | Record member contributions via Moolre Payments API |
| Payout Processing         | ✅ Complete | Automatic group payouts via Moolre Transfers API    |
| Trust Score Calculation   | ✅ Complete | Automatic calculation from payment history          |
| Trust Score History       | ✅ Complete | Track score changes over time                       |
| Savings Goals             | ✅ Complete | Personal savings with strict/loose modes            |
| Savings Deposits          | ✅ Complete | Add money to savings goals                          |
| Savings Withdrawals       | ✅ Complete | Withdraw (with strict mode enforcement)             |
| Transaction History       | ✅ Complete | Full audit trail of all transactions                |
| Member Profile Update     | ✅ Complete | Update name and profile picture                     |
| USSD State Machine        | ✅ Complete | Interactive menu system for feature phones          |
| Moolre Collections API    | ✅ Complete | Real payment requests with OTP verification         |
| Moolre Disbursements API  | ✅ Complete | Real payouts to mobile money wallets                |
| Moolre SMS API            | ✅ Complete | Send OTP and notifications via SMS                  |
| Webhook Handler           | ✅ Complete | Receive payment confirmations                       |
| Database Schema           | ✅ Complete | 9 tables with proper relationships                  |
| Database Migrations       | ✅ Complete | All migrations applied                              |
| Health Check              | ✅ Complete | Service health monitoring                           |

### Frontend (React + TypeScript)

| Feature                 | Status      | Description                                |
| ----------------------- | ----------- | ------------------------------------------ |
| Admin Dashboard         | ✅ Complete | Overview of groups, members, contributions |
| Admin Login             | ✅ Complete | Email/password authentication              |
| Admin Groups Management | ✅ Complete | View groups, members, trust scores         |
| Admin Payout Trigger    | ✅ Complete | One-click group payouts                    |
| Admin Transactions View | ✅ Complete | Full transaction history                   |
| Admin Settings          | ✅ Complete | API URL configuration, theme toggle        |
| Member Login            | ✅ Complete | Phone + OTP verification                   |
| Member Dashboard        | ✅ Complete | Savings overview, recent contributions     |
| Member Groups           | ✅ Complete | View groups, join groups                   |
| Member Group Detail     | ✅ Complete | View members, turn positions, contribute   |
| Member Savings Goals    | ✅ Complete | Create, view, deposit, withdraw            |
| Member Profile          | ✅ Complete | Update name, upload profile picture        |
| Member Trust Score      | ✅ Complete | View current score and history             |
| Member Transactions     | ✅ Complete | Full transaction history with filters      |
| USSD Simulator          | ✅ Complete | Terminal-style USSD demo                   |
| Dark/Light Theme        | ✅ Complete | Full theme support                         |
| Toast Notifications     | ✅ Complete | User feedback for all actions              |
| Loading States          | ✅ Complete | Skeletons for all data fetching            |
| Error Boundaries        | ✅ Complete | Graceful error handling                    |
| Responsive Design       | ✅ Complete | Mobile and desktop support                 |

### Moolre Integration

| API                | Status      | Purpose                      |
| ------------------ | ----------- | ---------------------------- |
| Collections API    | ✅ Complete | Accept payments from members |
| Disbursements API  | ✅ Complete | Send payouts to members      |
| SMS API            | ✅ Complete | Send OTP and notifications   |
| Validate Name API  | ✅ Complete | Verify account holder names  |
| Account Status API | ✅ Complete | Check wallet balance         |
| Transfers API      | ✅ Complete | Internal transfers           |
| Live Environment   | ✅ Complete | Production-ready integration |

---

## What's Yet to Build

### High Priority (Before Hackathon Submission)

| Feature                    | Description                                             | Estimated Time    |
| -------------------------- | ------------------------------------------------------- | ----------------- |
| Admin Create Group UI      | Admin can create groups from frontend                   | 2 hours           |
| Admin Add Member UI        | Admin can add members to groups                         | 2 hours           |
| Admin Delete Group/Member  | Admin can remove groups and members                     | 1 hour            |
| Contribution Status Update | Update from pending to completed after OTP verification | 2 hours           |
| Trust Score Auto-Update    | Recalculate trust score after each contribution         | 1 hour            |
| SMS Sender ID Approval     | Get "TrustLedger" Sender ID approved                    | Waiting on Moolre |
| Deploy Backend             | Deploy to Render or Railway                             | 2 hours           |
| Deploy Frontend            | Deploy to Vercel or Netlify                             | 1 hour            |
| Record Demo Video          | 3-minute walkthrough                                    | 2 hours           |
| Submit to Hackathon        | Complete submission form                                | 30 minutes        |

### Medium Priority (Post-Hackathon)

| Feature                   | Description                        |
| ------------------------- | ---------------------------------- |
| Admin Analytics Dashboard | Charts, reports, insights          |
| Admin Export Data         | CSV exports of transactions        |
| Member Notifications      | Push/SMS notifications for payouts |
| Referral System           | Invite members to groups           |
| Group Chat                | Real-time chat for group members   |
| QR Code Checkout          | Generate and scan QR codes         |
| Multiple Wallet Support   | Add support for other currencies   |
| OAuth Login               | Google, Facebook login             |
| WebSocket Real-time       | Live updates without refresh       |
| API Rate Limiting         | Prevent abuse                      |
| Production Logging        | Structured logging for monitoring  |

### Low Priority (Future Features)

| Feature               | Description                       |
| --------------------- | --------------------------------- |
| AI Fraud Detection    | Identify suspicious patterns      |
| Credit Scoring API    | Expose trust scores to lenders    |
| Insurance Integration | Micro-insurance based on trust    |
| Mobile App            | React Native app for members      |
| Agent Network         | Recruit agents to onboard users   |
| Loan Products         | Micro-loans based on trust scores |
| Savings Challenges    | Gamified savings competitions     |
| Financial Education   | In-app financial literacy content |

---

## Technical Architecture

### Frontend Stack

| Technology     | Purpose                   |
| -------------- | ------------------------- |
| React 18       | UI framework              |
| TypeScript     | Type safety               |
| Tailwind CSS   | Styling                   |
| TanStack Query | Data fetching and caching |
| React Router   | Navigation                |
| Axios          | HTTP client               |
| Recharts       | Charts and graphs         |
| Lucide React   | Icons                     |

### Backend Stack

| Technology     | Purpose              |
| -------------- | -------------------- |
| Rust           | Programming language |
| Loco Framework | Web framework        |
| SeaORM         | ORM                  |
| PostgreSQL     | Database             |
| JWT            | Authentication       |
| bcrypt         | Password hashing     |
| Axum           | HTTP routing         |
| Serde          | JSON serialization   |

### Third-Party Integrations

| Service              | Purpose                       |
| -------------------- | ----------------------------- |
| Moolre               | Payment processing, SMS, USSD |
| Moolre Collections   | Accept payments               |
| Moolre Disbursements | Send payouts                  |
| Moolre SMS           | Send OTP and notifications    |

---

## Deployment Status

| Environment | Status             | URL                                                                |
| ----------- | ------------------ | ------------------------------------------------------------------ |
| Development | ✅ Running locally | <http://localhost:5150> (backend) / <http://localhost:5173> (frontend) |
| Production  | ⏳ Not deployed    | TBD                                                                |

---

## Live API Environment

TrustLedger uses **Moolre Live API** (`https://api.moolre.com`) for all payment flows:

| Feature       | Live Status                      |
| ------------- | -------------------------------- |
| Collections   | ✅ Working (OTP sent to phone)   |
| Disbursements | ✅ Working (real money movement) |
| SMS           | ✅ Working (real SMS)            |
| USSD          | ⏳ Requires shortcode purchase   |

---

## Next Steps for Hackathon

1. **Get SMS Sender ID Approved** → "TrustLedger" (waiting on Moolre)
2. **Deploy Backend** → Render/Railway
3. **Deploy Frontend** → Vercel/Netlify
4. **Record Demo Video** → 3-minute walkthrough
5. **Submit to Hackathon** → Before July 13 deadline

---

## License

© 2026 TrustLedger. All rights reserved.
