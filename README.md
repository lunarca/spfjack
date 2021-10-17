# SPFJack

Email spoofing is dead, but misconfiguration never dies.


## Purpose

This project is designed to take in domain names and review their SPF records for any issues that could result in email spoofing becoming possible. 

The list of planned misconfigurations is as follows:
- `+all` mechanism existing.
- `ip4`, `ip6`, or `a` mechanisms with hosts containing open SMTP relays.
- `a`, `mx`, and `ptr` mechanisms for domains that are not registered.
- `exists` mechanisms existing.
- `include` mechanisms for domains that are not registered. Recursively evaluate the SPF record for the `include` specified domain.
- `redirect` modifier for domains that are not registered. Recursively evaluate the SPF record for the `redirect` specified domain.