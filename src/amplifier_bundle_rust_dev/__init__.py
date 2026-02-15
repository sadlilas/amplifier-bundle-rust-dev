"""Amplifier Rust Development Bundle.

Provides comprehensive Rust development tools including:
- Code quality checks (cargo fmt, clippy, cargo check)
- Stub/placeholder detection
- Integration with Amplifier as tool and hook modules
"""

from .checker import RustChecker, check_content, check_files
from .models import CheckConfig, CheckResult, Issue, Severity

__version__ = "0.1.0"

__all__ = [
    "RustChecker",
    "check_files",
    "check_content",
    "CheckResult",
    "Issue",
    "Severity",
    "CheckConfig",
]
