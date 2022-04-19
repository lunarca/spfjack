# SPFJack

Email spoofing is dead, but misconfiguration never dies.


## Purpose

This project is designed to take in domain names and review their SPF records for any issues that could result in email spoofing becoming possible. 

The list of planned misconfigurations is as follows:
- No SPF record existing. Without an SPF record, an MTA can't determine if an email is legitimately sent.
    - Note: This misconfiguration may need testing to see if it's accurate.
- `+all` mechanism existing.
- `ip4`, `ip6`, or `a` mechanisms with hosts containing open SMTP relays.
- `a`, `mx`, and `ptr` mechanisms for domains that are not registered.
- `exists` mechanisms existing. This can be a bit more complicated, see the [SPF Domain spec](https://datatracker.ietf.org/doc/html/rfc7208#section-7.1) and the `_spf.salesforce.com` record
- `include` mechanisms for domains that are not registered. Recursively evaluate the SPF record for the `include` specified domain.
- `redirect` modifier for domains that are not registered. Recursively evaluate the SPF record for the `redirect` specified domain.
- DMARC record with `p=none` -- This allows for From/SendFrom confusion.