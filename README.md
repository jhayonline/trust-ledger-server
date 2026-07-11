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
| Join group       | Dial *713# → enter group code     |
| Pay contribution | USSD menu → amount → Momo payment |
| Check balance    | Dial *713# → "My Balance"         |
| View trust score | Dial *713# → "My Trust Score"     |

No smartphone required. Works on any phone.

### For Group Admins (via Web Dashboard)

| Feature             | What they see                                        |
| ------------------- | ---------------------------------------------------- |
| Group overview      | Members, total saved, next payout date               |
| Member trust scores | Color-coded indicators with explanations             |
| Transaction history | Every contribution recorded permanently              |
| One-click payouts   | Automatic disbursement to all members via Moolre API |
| Create Group        | Create new susu groups from the dashboard            |
| Add Member          | Add members to existing groups                       |
| Delete Group        | Remove groups and all associated data                |
| Delete Member       | Remove members from groups                           |

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

(On-time payments / Expected payments) × 100

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

| Feature                   | Status   | Description                                         |
| ------------------------- | -------- | --------------------------------------------------- |
| Admin Authentication      | Complete | JWT-based login with bcrypt                         |
| Member OTP Authentication | Complete | Phone number + OTP via SMS (live Moolre SMS API)    |
| Group Management          | Complete | Create, list, view, delete groups                   |
| Member Management         | Complete | Add, list, delete members with trust scores         |
| Contribution Recording    | Complete | Record member contributions via Moolre Payments API |
| Contribution Verification | Complete | Verify contributions with OTP                       |
| Payout Processing         | Complete | Automatic group payouts via Moolre Transfers API    |
| Trust Score Calculation   | Complete | Automatic calculation from payment history          |
| Trust Score History       | Complete | Track score changes over time                       |
| Savings Goals             | Complete | Personal savings with strict/loose modes            |
| Savings Deposits          | Complete | Add money to savings goals                          |
| Savings Withdrawals       | Complete | Withdraw (with strict mode enforcement)             |
| Transaction History       | Complete | Full audit trail of all transactions                |
| Member Profile Update     | Complete | Update name and profile picture                     |
| Profile Picture Upload    | Complete | Upload and store profile pictures                   |
| USSD State Machine        | Complete | Interactive menu system for feature phones          |
| Moolre Collections API    | Complete | Real payment requests with OTP verification         |
| Moolre Disbursements API  | Complete | Real payouts to mobile money wallets                |
| Moolre SMS API            | Complete | Send OTP and notifications via SMS                  |
| Webhook Handler           | Complete | Receive payment confirmations                       |
| Database Schema           | Complete | 9 tables with proper relationships                  |
| Database Migrations       | Complete | All migrations applied                              |
| Database Seeding          | Complete | YAML fixtures for development data                  |
| Health Check              | Complete | Service health monitoring                           |

### Frontend (React + TypeScript)

| Feature                  | Status   | Description                                |
| ------------------------ | -------- | ------------------------------------------ |
| Admin Dashboard          | Complete | Overview of groups, members, contributions |
| Admin Login              | Complete | Email/password authentication              |
| Admin Groups Management  | Complete | View, create, delete groups                |
| Admin Members Management | Complete | View, add, delete members                  |
| Admin Payout Trigger     | Complete | One-click group payouts                    |
| Admin Transactions View  | Complete | Full transaction history                   |
| Admin Settings           | Complete | API URL configuration, theme toggle        |
| Member Login             | Complete | Phone + OTP verification                   |
| Member Dashboard         | Complete | Savings overview, recent contributions     |
| Member Groups            | Complete | View groups, join groups                   |
| Member Group Detail      | Complete | View members, turn positions, contribute   |
| Member Savings Goals     | Complete | Create, view, deposit, withdraw            |
| Member Profile           | Complete | Update name, upload profile picture        |
| Member Trust Score       | Complete | View current score and history             |
| Member Transactions      | Complete | Full transaction history with filters      |
| USSD Simulator           | Complete | Terminal-style USSD demo                   |
| Dark/Light Theme         | Complete | Full theme support                         |
| Toast Notifications      | Complete | User feedback for all actions              |
| Loading States           | Complete | Skeletons for all data fetching            |
| Error Boundaries         | Complete | Graceful error handling                    |
| Responsive Design        | Complete | Mobile and desktop support                 |

### Moolre Integration

| API                | Status   | Purpose                      |
| ------------------ | -------- | ---------------------------- |
| Collections API    | Complete | Accept payments from members |
| Disbursements API  | Complete | Send payouts to members      |
| SMS API            | Complete | Send OTP and notifications   |
| Validate Name API  | Complete | Verify account holder names  |
| Account Status API | Complete | Check wallet balance         |
| Transfers API      | Complete | Internal transfers           |
| Live Environment   | Complete | Production-ready integration |

---

## What's Yet to Build

### High Priority (Before Hackathon Submission)

| Feature             | Description                 | Estimated Time |
| ------------------- | --------------------------- | -------------- |
| Deploy Backend      | Deploy to Render or Railway | 2 hours        |
| Deploy Frontend     | Deploy to Vercel or Netlify | 1 hour         |
| Record Demo Video   | 3-minute walkthrough        | 2 hours        |
| Submit to Hackathon | Complete submission form    | 30 minutes     |

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

| Environment | Status          | URL                                                                |
| ----------- | --------------- | ------------------------------------------------------------------ |
| Development | Running locally | <http://localhost:5150> (backend) / <http://localhost:5173> (frontend) |
| Production  | Not deployed    | TBD                                                                |

---

## Live API Environment

TrustLedger uses **Moolre Live API** (<https://api.moolre.com>) for all payment flows:

| Feature       | Live Status                   |
| ------------- | ----------------------------- |
| Collections   | Working (OTP sent to phone)   |
| Disbursements | Working (real money movement) |
| SMS           | Working (real SMS)            |
| USSD          | Requires shortcode purchase   |

---

## Database Schema

### Users

Stores admin users for dashboard access.

### Groups

Represents susu groups with members, contributions, and payout schedules.

### Members

Individual members with trust scores and contribution history.

### Contributions

Records each payment made by a member to a group.

### Transactions

Full audit trail of all financial movements (contributions and payouts).

### Savings Goals

Personal savings targets with strict/loose withdrawal modes.

### Savings Deposits

Individual deposits made toward savings goals.

### Trust Score History

Tracks changes in trust scores over time with reasons.

---

## API Endpoints (Key Routes)

| Method | Endpoint                          | Description                  |
| ------ | --------------------------------- | ---------------------------- |
| POST   | /api/auth/login                   | Admin login                  |
| POST   | /api/member/auth/send-otp         | Request OTP for member login |
| POST   | /api/member/auth/verify           | Verify OTP and get token     |
| GET    | /api/groups                       | List all groups              |
| POST   | /api/groups                       | Create group (admin)         |
| DELETE | /api/groups/{id}                  | Delete group (admin)         |
| DELETE | /api/members/{id}                 | Delete member (admin)        |
| GET    | /api/groups/{id}/members          | List members in group        |
| POST   | /api/groups/{id}/members          | Add member to group (admin)  |
| POST   | /api/groups/{id}/payout           | Trigger group payout         |
| POST   | /api/contributions/verify         | Verify contribution with OTP |
| GET    | /api/member/dashboard             | Member dashboard data        |
| GET    | /api/member/groups                | Member's groups              |
| POST   | /api/member/groups/join           | Join a group                 |
| POST   | /api/member/contribute            | Make a contribution          |
| GET    | /api/member/savings               | List savings goals           |
| POST   | /api/member/savings               | Create savings goal          |
| POST   | /api/member/savings/deposit       | Deposit to savings goal      |
| POST   | /api/member/savings/withdraw/{id} | Withdraw from savings        |
| GET    | /api/member/trust-score/history   | Trust score history          |
| PUT    | /api/member/profile               | Update profile               |
| PUT    | /api/member/profile/picture       | Upload profile picture       |
| GET    | /api/member/transactions          | Member transaction history   |
| POST   | /api/ussd                         | USSD menu handler            |
| GET    | /api/health                       | Service health check         |

---

## Quick Start

### Prerequisites

- Rust 1.80+
- PostgreSQL 15+
- Node.js 20+
- Moolre API credentials

### Backend Setup

````bash
cd ~/software/rust/loco/trust-ledger-server
cargo run

### Frontend Setup

```bash
cd ~/software/typescript/react/trust-ledger-client
npm install
npm run dev
````

### Seed Database

```bash
cargo loco db seed
```

Or run the SQL seed file:

```bash
psql -U postgres -d trust_ledger_server -f sql_statements/seed_data.sql
```

### Default Admin Credentials

| Email                 | Password |
| --------------------- | -------- |
| <admin@trustledger.com> | admin123 |

---

## Next Steps for Hackathon

1. Deploy Backend to Render/Railway
2. Deploy Frontend to Vercel/Netlify
3. Record Demo Video (3-minute walkthrough)
4. Submit to Hackathon (July 13 deadline)

---

## License

2026 TrustLedger. All rights reserved.
