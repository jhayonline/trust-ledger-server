# TrustLedger - Missing Features & Bug Tracker

> Generated: 2026-07-11
> Status: Pre-deployment Review

---

## 🔴 CRITICAL BUGS

### 1. Member ID vs JWT Claim Mismatch

**Files:** `member_groups.rs`, `member_dashboard.rs`, `member_savings.rs`, `member_trust_score.rs`, `member_transactions.rs`, `member_profile.rs`, `member_contribute.rs`

**Issue:** `auth.claims.pid` (UUID) is being parsed as `i32` but member IDs are integers from the members table. These are completely different entities.

---

### 2. Trust Score Not Recalculated

**Files:** `member_contribute.rs`, `payout.rs`, `member_savings.rs`

**Issue:** Trust scores are set to 100 on member creation but never updated after contributions, payouts, or missed payments. Trust scores don't reflect actual payment history.

---

### 3. Webhook Handler Not Implemented

**File:** `webhooks.rs`

**Issue:** Moolre callbacks are received but not processed. Contributions remain in "pending" state forever and payment confirmations are never recorded.

---

### 4. Missing Database Indexes

**Files:** Migration files

**Issue:** No indexes on foreign keys (`member_id`, `group_id`) or frequently queried columns (`phone`, `date`, `status`). Performance will degrade as data grows.

---

### 5. Group Payout - Missing Individual Payout Tracking

**File:** `payout.rs`

**Issue:** `total_contributed` is reset to 0 for all members but no historical record of individual payouts is kept. Cannot track member payout history or calculate lifetime earnings.

---

## 🟡 HIGH PRIORITY MISSING FEATURES

### 6. Member Registration Flow

**File:** `member_auth.rs`

**Issue:** Members are created automatically on OTP request with auto-generated names (`Member_XXXX`). No proper registration with name, optional password, or welcome flow.

---

### 7. Contribution Limits Per Cycle

**File:** `member_contribute.rs`

**Issue:** Members can contribute multiple times per cycle (weekly/monthly). Should only allow one contribution per cycle.

---

### 8. Savings Goal Withdrawals - Missing Transaction Record

**File:** `member_savings.rs`

**Issue:** When withdrawing from savings, the goal is updated but no transaction record is created in the transactions table.

---

### 9. Trust Score - Missing "Reason" Tracking

**File:** `member_trust_score.rs`

**Issue:** Trust score changes are recorded with generic reasons. Need specific reasons like "On-time contribution", "Missed payment", "Joined group", etc.

---

### 10. Profile Picture Upload - No File Validation

**File:** `member_profile.rs`

**Issue:** Base64 images are accepted without validation. No file size limits (should be <2MB), no image type validation (should be JPEG/PNG only), no XSS sanitization.

---

### 11. OTP SMS Delivery Tracking

**File:** `member_auth.rs`

**Issue:** SMS sending is not tracked for delivery status. No retry mechanism if SMS fails. No record of which SMS reference was sent.

---

### 12. Payout Scheduling - No Automatic Check

**Files:** `tasks/mod.rs`

**Issue:** Groups have `next_payout_date` but no automatic scheduler to check for due payouts. Admins must manually trigger every payout.

---

## 🟠 MEDIUM PRIORITY MISSING FEATURES

### 13. Group Invite System

**New File Needed:** `invites.rs`

**Issue:** No way for admins to invite members via SMS with a group code or link.

---

### 14. Member Onboarding Flow

**File:** `member_auth.rs`

**Issue:** No step-by-step onboarding for new members showing them how to join groups, contribute, and use the app.

---

### 15. Admin Analytics Dashboard

**File:** `dashboard.rs`

**Issue:** Only basic stats available. No charts for contribution trends, member growth, trust score distribution, or payout history.

---

### 16. Member Notifications

**Files:** `member_dashboard.rs`

**Issue:** Members don't receive notifications for:

- Payout completions
- Upcoming contribution deadlines
- Trust score changes
- Group announcements

---

### 17. Contribution History - No Export

**File:** `member_transactions.rs`

**Issue:** No CSV/PDF export for transaction history.

---

### 18. Admin - Bulk Member Addition

**File:** `groups.rs`

**Issue:** Members must be added one by one. No CSV import or bulk add feature.

---

### 19. Rate Limiting

**All API endpoints**

**Issue:** No rate limiting on authentication endpoints. OTP requests could be abused.

---

### 20. API Versioning

**All API endpoints**

**Issue:** No API versioning (`/api/v1/`). Breaking changes will affect all clients.

---

## 🟣 LOW PRIORITY MISSING FEATURES

### 21. QR Code Checkout

**New Feature**

**Issue:** No QR code generation for members to scan and contribute.

---

### 22. Multiple Wallet Support

**Backend**

**Issue:** Only supports GHS. No multi-currency support.

---

### 23. OAuth Login

**File:** `auth.rs`

**Issue:** No Google/Facebook login option.

---

### 24. Real-time WebSocket Updates

**New Feature**

**Issue:** Dashboard updates require page refresh. No WebSocket for live updates.

---

### 25. Group Chat

**New Feature**

**Issue:** No real-time chat for group members.

---

### 26. Referral System

**New Feature**

**Issue:** No referral tracking or rewards for inviting new members.

---

### 27. Push Notifications

**File:** `member_dashboard.rs`

**Issue:** No mobile push notifications for Android/iOS.

---

### 28. Analytics - Export Reports

**File:** `dashboard.rs`

**Issue:** No PDF/Excel report generation for admins.

---

### 29. Audit Log

**New Feature**

**Issue:** No audit trail of admin actions (who deleted what, when).

---

### 30. Backup & Restore

**Backend**

**Issue:** No automated backup or restore functionality.

---

## 🐛 FRONTEND BUGS & ISSUES

### 31. Date Formatting Inconsistency

**All frontend pages**

**Issue:** Mix of date formats across the app. Some use `en-GB`, some use default locale.

---

### 32. Toast Notifications - Missing Loading States

**All frontend pages**

**Issue:** No loading indicators during long operations like contributions or payouts.

---

### 33. Mobile Responsiveness - USSD Simulator

**File:** `UssdSimulator.tsx`

**Issue:** USSD simulator has fixed height `h-[520px]` and doesn't resize well on mobile.

---

### 34. Keyboard Accessibility

**File:** `Modal.tsx`

**Issue:** Modals don't handle Escape key press to close.

---

### 35. Empty State Handling

**Files:** `MemberDashboardPage.tsx`, `MemberGroupsPage.tsx`, `MemberSavingsPage.tsx`, `TransactionsPage.tsx`

**Issue:** Some pages show blank states without helpful messages when no data exists.

---

### 36. Error Boundary Missing

**App.tsx**

**Issue:** No error boundary for React component crashes. The whole app crashes on unhandled errors.

---

### 37. Form Validation - No Real-time Feedback

**All forms**

**Issue:** Form validation only on submit. No real-time feedback on input fields.

---

### 38. Profile Picture - No Preview

**File:** `MemberProfilePage.tsx`

**Issue:** No image preview before uploading profile picture.

---

### 39. Missing Progress Indicators

**All pages**

**Issue:** No progress indicators for file uploads or long-running operations.

---

### 40. Pagination - No Page Size Persistence

**File:** `TransactionsPage.tsx`

**Issue:** Page size resets to default when navigating away.

---

## 📊 DATABASE ISSUES

### 41. Missing Soft Delete

**Tables:** `members`, `groups`

**Issue:** Hard delete removes all records. Should use `is_deleted` flag for soft delete.

---

### 42. Missing Created/Updated Tracking

**Tables:** `contributions`, `transactions`, `savings_deposits`

**Issue:** No `created_at` or `updated_at` timestamps on some tables.

---

### 43. Missing Foreign Key Constraints

**Tables:** Various

**Issue:** Some relationships defined in code but not enforced at database level with proper foreign key constraints.

---

### 44. Missing Audit Trail

**All tables**

**Issue:** No `created_by` or `updated_by` columns to track which admin/member made changes.

---

### 45. Missing Search Indexes

**Tables:** `members`, `groups`, `transactions`

**Issue:** No full-text search indexes for searching by name, phone, or description.

---

## 🔧 INFRASTRUCTURE & DEV OPS

### 46. No CI/CD Pipeline

**Project Root**

**Issue:** No GitHub Actions or CI/CD configured for automated testing and deployment.

---

### 47. No Docker Configuration

**Project Root**

**Issue:** No Dockerfile or docker-compose.yml for containerized deployment.

---

### 48. No Environment Configuration Validation

**File:** `config/`

**Issue:** No validation that all required environment variables are set at startup.

---

### 49. Missing Health Check - Detailed

**File:** `health.rs`

**Issue:** Health check only returns status. Should check database connectivity and Moolre API availability.

---

### 50. No Logging Aggregation

**Backend**

**Issue:** No structured logging or log aggregation for production monitoring.

---

## Summary Table

| Category                    | Count  |
| --------------------------- | ------ |
| 🔴 Critical Bugs            | 5      |
| 🟡 High Priority Features   | 7      |
| 🟠 Medium Priority Features | 8      |
| 🟣 Low Priority Features    | 10     |
| 🐛 Frontend Bugs/Issues     | 10     |
| 📊 Database Issues          | 5      |
| 🔧 Infrastructure Issues    | 5      |
| **TOTAL**                   | **50** |

---

## Quick Fix Priority (Before Hackathon Submission)

| Must Fix Now                | Should Fix                          | Nice to Have             |
| --------------------------- | ----------------------------------- | ------------------------ |
| ✅ Bug #1 (JWT mismatch)    | ✅ Feature #6 (Registration)        | Feature #13 (Invites)    |
| ✅ Bug #2 (Trust score)     | ✅ Feature #7 (Contribution limits) | Feature #14 (Onboarding) |
| ✅ Bug #3 (Webhook)         | ✅ Feature #8 (Transaction record)  | Feature #15 (Analytics)  |
| ✅ Bug #4 (Indexes)         | ✅ Feature #10 (Profile validation) | Feature #21 (QR codes)   |
| ✅ Bug #5 (Payout tracking) | ✅ Bug #31 (Date formatting)        | Feature #27 (Push)       |
